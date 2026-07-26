mod cli;
mod schema;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};
use il2cpp_runtime::{Il2CppMetadata, ProcessMemory, RuntimeIntrospector};
use schema::SchemaEngine;
use std::time::Instant;

fn main() -> Result<()> {
    let started = Instant::now();
    let cli = Cli::parse();

    let pid = resolve_pid(cli.pid)?;

    let memory = ProcessMemory::new(pid)?;
    let mut introspector = RuntimeIntrospector::new(memory);

    println!("Finding Il2Cpp metadata...");
    let _metadata = Il2CppMetadata::find_in_process(introspector.process_memory(), pid)?;
    println!("  Found at {:#x}\n", _metadata.registration_addr);

    let mut engine = SchemaEngine::new(introspector);

    match cli.command {
        Commands::Schema(args) => {
            if args.interactive {
                engine.interactive_schema(&args.follow)?;
            } else if args.all_types {
                let schema = engine.dump_all_types_schema()?;
                let output = args
                    .output
                    .clone()
                    .unwrap_or_else(|| std::path::PathBuf::from(args.default_output()));
                let json = if args.compact {
                    serde_json::to_string(&schema)?
                } else {
                    serde_json::to_string_pretty(&schema)?
                };
                std::fs::write(&output, json)?;
                println!("\n✓ Dumped {} metadata types", schema.total_types);
                println!("  Wrote {}", output.display());
            } else {
                let root_ptr: u64 = if let Some(sel) = &args.root {
                    println!("Selecting root singleton '{}'...", sel);
                    engine.resolve_singleton_root(sel)?
                } else {
                    engine.resolve_work_data_manager_instance()?
                };

                let mut object_ptr = root_ptr;
                let mut resolved_follow_path = Vec::new();
                for step in &args.follow {
                    let (child, step_name) = engine.follow_step(object_ptr, step)?;
                    println!("  -> Followed '{}' → {:#x}", step_name, child);
                    resolved_follow_path.push(step_name);
                    object_ptr = child;
                }

                let parsed_peeks: Vec<(String, Option<String>)> = args
                    .peek
                    .iter()
                    .map(|s| {
                        if let Some((field, decoder)) = s.split_once(':') {
                            (field.to_string(), Some(decoder.to_string()))
                        } else {
                            (s.clone(), None)
                        }
                    })
                    .collect();

                let dump_schema = args.output.is_some()
                    || (parsed_peeks.is_empty() && args.peek_output.is_none());

                if dump_schema {
                    let schema = engine.build_schema_dump_for_object(
                        object_ptr,
                        resolved_follow_path.clone(),
                        &args.fields,
                    )?;
                    let output = args
                        .output
                        .clone()
                        .unwrap_or_else(|| std::path::PathBuf::from(args.default_output()));
                    let json = if args.compact {
                        serde_json::to_string(&schema)?
                    } else {
                        serde_json::to_string_pretty(&schema)?
                    };
                    std::fs::write(&output, json)?;
                    println!("\n✓ Dumped {} fields", schema.fields.len());
                    println!("  Wrote {}", output.display());
                }

                if !parsed_peeks.is_empty() {
                    let peek_result =
                        engine.batch_peek(object_ptr, &parsed_peeks, &resolved_follow_path)?;
                    if let Some(peek_path) = &args.peek_output {
                        let peek_json = serde_json::to_string_pretty(&peek_result)?;
                        std::fs::write(peek_path, peek_json)?;
                        println!("  Wrote peek results to {}", peek_path.display());
                    }
                }
            }
        }
    }

    println!("\nDone in {}", format_elapsed(started.elapsed()));
    Ok(())
}

fn resolve_pid(pid: Option<u32>) -> Result<u32> {
    match pid {
        Some(pid) => {
            println!("Using explicit PID: {}", pid);
            Ok(pid)
        }
        None => {
            println!("Finding game process...");
            let process = shared::process::find_game_process()?;
            println!("  Found: {} (PID: {})", process.name, process.pid);
            Ok(process.pid)
        }
    }
}

fn format_elapsed(d: std::time::Duration) -> String {
    let s = d.as_secs();
    let ms = d.subsec_millis();
    if s == 0 {
        format!("{} ms", ms)
    } else {
        format!("{}.{:03} s", s, ms)
    }
}

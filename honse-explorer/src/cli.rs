use clap::{Args, Parser};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "il2cpp-runtime",
    version,
    about = "Schema dump and interactive runtime field explorer for IL2CPP games"
)]
pub struct Cli {
    /// Target process PID; if omitted, auto-detect the game process
    #[arg(long)]
    pub pid: Option<u32>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(clap::Subcommand)]
pub enum Commands {
    /// Schema dump mode — discover all fields per class
    Schema(SchemaArgs),
}

#[derive(Args)]
pub struct SchemaArgs {
    /// Restrict output to one or more specific field names (repeatable)
    #[arg(long = "field")]
    pub fields: Vec<String>,

    /// Follow a pointer field into a child object (repeatable; applied in order).
    /// Special steps: @dict-value, @list-item, @array-item
    #[arg(short = 'F', long = "follow")]
    pub follow: Vec<String>,

    /// Output JSON file path
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Write compact JSON instead of pretty-printed JSON
    #[arg(long, default_value_t = false)]
    pub compact: bool,

    /// Dump all metadata type definitions and their fields from global-metadata.dat
    #[arg(long, default_value_t = false)]
    pub all_types: bool,

    /// Root singleton class name or index (e.g., "Gallop::WorkDataManager" or "3")
    #[arg(long, value_name = "SELECTOR")]
    pub root: Option<String>,

    /// Peek a field value at the final object with an optional decoder.
    /// Format: field_name[:decoder] (repeatable)
    #[arg(long, value_name = "FIELD[:DECODER]")]
    pub peek: Vec<String>,

    /// Write peek results as JSON to this file
    #[arg(long, value_name = "FILE")]
    pub peek_output: Option<PathBuf>,

    /// Start an interactive schema exploration REPL
    #[arg(short = 'i', long, default_value_t = false)]
    pub interactive: bool,

    /// Include non-public fields in the dump
    #[arg(long, default_value_t = false)]
    pub include_private: bool,

    /// Verbose logging
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,
}

impl SchemaArgs {
    pub fn default_output(&self) -> &'static str {
        if self.all_types {
            "all-types-schema.json"
        } else {
            "schema.json"
        }
    }
}

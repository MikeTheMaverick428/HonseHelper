use crate::config::DEFAULT_MAX_SCAN_BYTES;
use crate::protocol::{
    read_request_msgpack_framed, ready_event, respond_err, respond_ok,
    write_response_msgpack_framed, WorkerCommand, WorkerRequest,
};
use crate::worker_state::WorkerState;
use anyhow::Result;
use std::io::{self, BufRead, Write};

pub fn run() -> Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut state = WorkerState::default();
    let mut stdin = io::BufReader::new(stdin.lock());

    write_response_msgpack_framed(&mut stdout, &ready_event("honse-worker"))?;
    stdout.flush()?;

    run_msgpack_loop(&mut stdin, &mut stdout, &mut state)?;

    Ok(())
}

fn run_msgpack_loop(
    stdin: &mut io::BufReader<io::StdinLock<'_>>,
    stdout: &mut io::Stdout,
    state: &mut WorkerState,
) -> Result<()> {
    loop {
        if stdin.fill_buf()?.is_empty() {
            break;
        }

        let req = match read_request_msgpack_framed(stdin) {
            Ok(Some(v)) => v,
            Ok(None) => break,
            Err(e) => {
                write_response_msgpack_framed(
                    stdout,
                    &respond_err(None, &format!("Invalid request: {}", e)),
                )?;
                stdout.flush()?;
                break;
            }
        };

        let should_quit = handle_request(state, req, stdout)?;
        if should_quit {
            break;
        }
    }

    Ok(())
}

fn handle_request(
    state: &mut WorkerState,
    req: WorkerRequest,
    stdout: &mut io::Stdout,
) -> Result<bool> {
    use rmpv::Value;
    let id = req.id;

    let response = match req.command {
        WorkerCommand::Ping => respond_ok(id, rmp_val([("pong", Value::Boolean(true))])),
        WorkerCommand::FindProcess => match state.ensure_process() {
            Ok(_) => {
                let proc = state
                    .process
                    .as_ref()
                    .expect("process set after ensure_process");
                respond_ok(
                    id,
                    rmp_val([
                        ("pid", Value::from(proc.pid)),
                        ("name", Value::from(proc.name.as_str())),
                        (
                            "path",
                            proc.path
                                .as_ref()
                                .map(|p| Value::from(p.display().to_string()))
                                .unwrap_or(Value::Nil),
                        ),
                    ]),
                )
            }
            Err(e) => respond_err(id, &e.to_string()),
        },
        WorkerCommand::GetViewState { max_scan_bytes } => {
            let max_scan_bytes = max_scan_bytes.unwrap_or(DEFAULT_MAX_SCAN_BYTES);
            let result = (|| {
                state.ensure_process()?;
                let scene_manager_ptr = state.discover_scene_manager(max_scan_bytes)?;
                state.ensure_scene_offsets(scene_manager_ptr)?;
                state.read_view_state()
            })();
            match result {
                Ok(payload) => respond_ok(id, payload),
                Err(e) => respond_err(id, &e.to_string()),
            }
        }
        WorkerCommand::GetVeteranData => {
            let result = (|| {
                state.ensure_process()?;
                state.extract_veteran_data()
            })();
            match result {
                Ok(payload) => respond_ok(id, payload),
                Err(e) => respond_err(id, &e.to_string()),
            }
        }
        WorkerCommand::GetFriendData => {
            let result = (|| {
                state.ensure_process()?;
                state.extract_friend_data()
            })();
            match result {
                Ok(payload) => respond_ok(id, payload),
                Err(e) => respond_err(id, &e.to_string()),
            }
        }
        WorkerCommand::GetSupportCardData => {
            let result = (|| {
                state.ensure_process()?;
                state.extract_support_card_data()
            })();
            match result {
                Ok(payload) => respond_ok(id, payload),
                Err(e) => respond_err(id, &e.to_string()),
            }
        }
        WorkerCommand::GetUserData => {
            let result = (|| {
                state.ensure_process()?;
                state.get_user_data()
            })();
            match result {
                Ok(payload) => {
                    let value = rmp_serde::from_slice(&rmp_serde::to_vec(&payload)?)?;
                    respond_ok(id, value)
                }
                Err(e) => respond_err(id, &e.to_string()),
            }
        }
        WorkerCommand::GetRaceTeamData => {
            let result = (|| {
                state.ensure_process()?;
                state.extract_race_team_data()
            })();
            match result {
                Ok(payload) => respond_ok(id, payload),
                Err(e) => respond_err(id, &e.to_string()),
            }
        }
        WorkerCommand::GetTrophyData => {
            let result = (|| {
                state.ensure_process()?;
                state.extract_trophy_data()
            })();
            match result {
                Ok(payload) => respond_ok(id, payload),
                Err(e) => respond_err(id, &e.to_string()),
            }
        }
        WorkerCommand::GetCardData => {
            let result = (|| {
                state.ensure_process()?;
                state.extract_card_data()
            })();
            match result {
                Ok(payload) => respond_ok(id, payload),
                Err(e) => respond_err(id, &e.to_string()),
            }
        }
        WorkerCommand::Disconnect => {
            *state = WorkerState::default();
            respond_ok(id, rmp_val([("disconnected", Value::Boolean(true))]))
        }
        WorkerCommand::Quit | WorkerCommand::Exit => {
            write_response_msgpack_framed(
                stdout,
                &respond_ok(id, rmp_val([("bye", Value::Boolean(true))])),
            )?;
            stdout.flush()?;
            return Ok(true);
        }
    };

    write_response_msgpack_framed(stdout, &response)?;
    stdout.flush()?;
    Ok(false)
}

/// Build a msgpack map from a fixed-size array of (key, value) pairs.
fn rmp_val<const N: usize>(pairs: [(&str, rmpv::Value); N]) -> rmpv::Value {
    rmpv::Value::Map(
        pairs
            .into_iter()
            .map(|(k, v)| (rmpv::Value::from(k), v))
            .collect(),
    )
}

use anyhow::Result;

mod config;
mod current_view;
mod models;
mod protocol;
mod server;
mod worker_state;

fn main() -> Result<()> {
    server::run()
}

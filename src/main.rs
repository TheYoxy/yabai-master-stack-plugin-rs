#![cfg_attr(debug_assertions, allow(dead_code, unused_variables))]

use crate::initialize_panic_handler::initialize_panic_handler;

mod cli;
mod initialize_panic_handler;
mod macros;
pub mod task;
mod yabai;

fn main() -> color_eyre::Result<()> {
    use crate::task::ymsp_task::YmspTask;
    use clap::Parser as _;
    use log::{debug, info};

    initialize_panic_handler()?;
    #[cfg(debug_assertions)]
    pretty_env_logger::init_timed();

    info!("Starting ymsp");
    debug!("Parsing CLI arguments");
    let args = cli::cli::Cli::parse();

    debug!("Running command");
    args.run()
}
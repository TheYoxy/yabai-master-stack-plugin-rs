#![feature(once_cell_try)]
#![cfg_attr(debug_assertions, allow(dead_code, unused_variables))]

pub mod cli;
mod dry_mode;
mod initialize_logging;
mod initialize_panic_handler;
mod macros;
mod task;
pub mod window_manager;
mod yabai;
mod trace_command;

fn main() -> color_eyre::Result<()> {
  use clap::Parser as _;
  use log::{debug, info};
  use task::handlers::completion::generate_completion;

  use crate::{
    initialize_panic_handler::initialize_panic_handler,
    task::{ymsp_task::YmspTask, Task},
    yabai::config::check_config_path_exists,
  };

  initialize_panic_handler()?;

  info!("Starting ymsp");
  debug!("Parsing CLI arguments");
  let args = cli::Cli::parse();
  if let Task::Completions(completion) = &args.task {
    return generate_completion(completion);
  }

  initialize_logging::initialize_logging()?;

  check_config_path_exists()?;

  debug!("Running command");
  args.run()
}

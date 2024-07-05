#![feature(once_cell_try)]
#![cfg_attr(debug_assertions, allow(dead_code, unused_variables))]

use crate::yabai::config::check_config_path_exists;

pub mod cli;
mod dry_mode;
mod initialize_panic_handler;
mod macros;
mod task;
mod yabai;

fn main() -> color_eyre::Result<()> {
  use clap::Parser as _;
  use log::{debug, info};

  use crate::{initialize_panic_handler::initialize_panic_handler, task::ymsp_task::YmspTask};

  initialize_panic_handler()?;
  #[cfg(debug_assertions)]
  pretty_env_logger::init();

  check_config_path_exists()?;

  info!("Starting ymsp");
  debug!("Parsing CLI arguments");
  let args = cli::Cli::parse();

  debug!("Running command");
  args.run()
}

use clap::{command, Parser};

use crate::task::{ymsp_task::YmspTask, Task};

#[derive(Parser, Debug)]
#[command(version, author, about, long_about = None)]
pub struct Cli {
  /// The yabai master stack plugin task to run
  #[command(subcommand)]
  task: Task,

  /// Do not actually run the task, just print what would be done
  #[arg(short = 'n', long)]
  dry_run: bool,
}

impl YmspTask for Cli {
  fn run(&self) -> color_eyre::Result<()> { self.task.run() }
}

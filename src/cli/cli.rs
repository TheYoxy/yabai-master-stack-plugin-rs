use crate::task::ymsp_task::YmspTask;
use crate::task::Task;
use clap::{command, Parser};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    task: Task,
}

impl YmspTask for Cli {
    fn run(&self) -> color_eyre::Result<()> {
        self.task.run()
    }
}

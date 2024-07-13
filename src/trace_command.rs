use std::process::{Command, Output};

use color_eyre::{Help, SectionExt};
use color_eyre::owo_colors::OwoColorize;
use log::debug;
use tracing::debug_span;

pub trait ExecTrace {
  fn trace(&mut self) -> &mut Self;
  fn trace_output(&mut self) -> color_eyre::Result<Output>;
}
fn get_command_str(command: &Command) -> String {
  let args = command.get_args().filter_map(|a| a.to_str()).collect::<Vec<_>>().join(" ");
  format!("{} {}", command.get_program().to_str().unwrap().cyan(), args.yellow())
}

impl ExecTrace for Command {
  fn trace(&mut self) -> &mut Self {
    let cmd = get_command_str(self);
    debug_span!("Running command {cmd}");
    debug!("Running command {cmd}");

    self
  }

  fn trace_output(&mut self) -> color_eyre::Result<Output> {
    let cmd = get_command_str(self);
    let note = cmd.header("Command: ");
    self.trace().output().with_note(|| note)
  }
}

use std::process::{Command, Output};

use color_eyre::{eyre::eyre, owo_colors::OwoColorize, Section, SectionExt};
use log::{debug, error};
use tracing::debug_span;

use crate::{print_bool, yabai::config::get_config};

#[deprecated]
pub trait RunCommand {
  fn run_command(&mut self) -> color_eyre::Result<()>;
  fn run_command_with_output(&mut self) -> color_eyre::Result<Output>;
}

impl RunCommand for Command {
  fn run_command(&mut self) -> color_eyre::Result<()> {
    handle_output_result(self)?;
    Ok(())
  }

  fn run_command_with_output(&mut self) -> color_eyre::Result<Output> { handle_output_result(self) }
}

pub fn get_yabai_command() -> color_eyre::Result<Command> {
  let config = get_config()?;
  Ok(Command::new(&config.yabai_path))
}

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

fn handle_output_result(command: &mut Command) -> color_eyre::Result<Output> {
  let output = command.trace().output()?;

  let code = output.status.code().ok_or(eyre!("unable to get status code for command output"))?;
  let status = print_bool!(output.status.success(), code, code);
  if !output.status.success() {
    let stdout = String::from_utf8(output.stdout)?;
    error!("stdout: {stdout}");
    let stderr = String::from_utf8(output.stderr)?;
    error!("stderr: {stderr}");

    Err(
      eyre!("command failed with status {status}")
        .with_suggestion(|| "Check the logs for more information.")
        .with_section(|| output.status.to_string().header("Status: "))
        .with_section(|| stdout.header("Stdout: "))
        .with_section(|| stderr.header("Stderr: ")),
    )
  } else {
    Ok(output)
  }
}

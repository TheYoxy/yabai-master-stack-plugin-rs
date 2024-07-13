use std::fmt::Formatter;

use color_eyre::{eyre::eyre, owo_colors::OwoColorize, Section, SectionExt};
use log::{error, trace};

use crate::{
  print_bool,
  trace_command::ExecTrace,
  yabai::command::{message::YabaiMessage, to_argument::ToArgument},
};

pub trait Runnable {
  fn run(&self) -> color_eyre::Result<std::process::Output>;
}

impl Runnable for YabaiMessage {
  fn run(&self) -> color_eyre::Result<std::process::Output> {
    let command = self.to_command_str()?;
    let args = command.split(' ').collect::<Vec<_>>();
    let program = args.first().unwrap();
    let args = args.iter().skip(1).collect::<Vec<_>>();
    let mut command = std::process::Command::new(program);
    command.args(args).trace().output().map_err(|e| e.into()).and_then(|output| {
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
    })
  }
}

trait ToCommand {
  fn to_command_str(&self) -> color_eyre::Result<String>;
}

impl ToCommand for YabaiMessage {
  fn to_command_str(&self) -> color_eyre::Result<String> {
    trace!("building command string for message: {:?}", self);
    Ok(format!("{} -m {}", self.command, self.message.to_argument()))
  }
}

impl std::fmt::Display for YabaiMessage {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{} {}", self.command.cyan(), self.message.to_argument().yellow())
  }
}

#[cfg(test)]
mod tests {
  use pretty_assertions::assert_eq;

  use super::*;
  use crate::yabai::command::window_selector::YabaiWindowSelector;

  #[test_log::test]
  fn test_current_window() {
    let message = YabaiMessage::current_window().focus(YabaiWindowSelector::Next).unwrap();
    assert_eq!(message.to_command_str().unwrap(), "yabai -m window --focus next");
  }

  #[test_log::test]
  fn test_first_window_focus_next() {
    let message = YabaiMessage::window(YabaiWindowSelector::First).focus(YabaiWindowSelector::Next).unwrap();
    assert_eq!(message.to_command_str().unwrap(), "yabai -m window first --focus next");
  }
}

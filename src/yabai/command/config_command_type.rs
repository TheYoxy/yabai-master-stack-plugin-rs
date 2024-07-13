use color_eyre::eyre::{Context, OptionExt};
use log::debug;

use crate::yabai::{
  command::{
    message::{YabaiMessage, YabaiMessageBuilder},
    message_type::YabaiMessageType,
    to_argument::ToArgument,
    to_command::Runnable,
  },
  config::get_config,
};

#[derive(Debug, Clone)]
pub enum YabaiConfigCommandType {
  LeftPadding,
}

impl YabaiMessageBuilder<(), YabaiConfigCommandType> {
  /// Build the YabaiMessage from the builder.
  fn build(&self) -> color_eyre::Result<YabaiMessage> {
    debug!("creating yabai message for config");
    let command = get_config().map(|config| config.yabai_path).unwrap_or("yabai".to_string());
    let message = self.message.as_ref().ok_or_eyre("no command set")?.clone();
    debug!("command: {}", command);
    Ok(YabaiMessage { command, message: YabaiMessageType::Config(message) })
  }

  pub fn left_padding(&mut self) -> color_eyre::Result<f64> {
    self.message = Some(YabaiConfigCommandType::LeftPadding);
    let message = self.build()?;
    let output = message.run()?;
    let string = String::from_utf8(output.stdout)?;
    string.parse().with_context(|| format!("failed to parse left padding: {}", string))
  }
}

impl ToArgument for YabaiConfigCommandType {
  fn to_argument(&self) -> String {
    match self {
      YabaiConfigCommandType::LeftPadding => "left_padding".to_string(),
    }
  }
}

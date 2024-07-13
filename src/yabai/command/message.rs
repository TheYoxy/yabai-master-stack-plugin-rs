use color_eyre::eyre::OptionExt;

use crate::yabai::{
  command::{
    config_command_type::YabaiConfigCommandType, display_command_type::YabaiDisplayCommandType,
    display_selector::YabaiDisplaySelector, message_type::YabaiMessageType, query_command_type::YabaiQueryCommandType,
    window_command_type::YabaiWindowCommandType, window_selector::YabaiWindowSelector,
  },
  config::get_config,
};

#[derive(Debug, Clone)]
pub struct YabaiMessage {
  /// The yabai path to use.
  pub(super) command: String,
  /// The message sent to yabai.
  pub(super) message: YabaiMessageType,
  /// Whether the message is a write command.
  pub(super) is_write: bool,
}

#[derive(Debug, Clone)]
pub struct YabaiMessageBuilder<Selector, Message> {
  pub(super) selector: Option<Selector>,
  pub(super) message: Option<Message>,
}

impl<Selector, Message> Default for YabaiMessageBuilder<Selector, Message> {
  fn default() -> Self { Self { selector: None, message: None } }
}

impl<Message: Into<YabaiMessageType> + Clone> YabaiMessageBuilder<(), Message> {
  /// Build the YabaiMessage from the builder.
  pub(super) fn build(&self) -> color_eyre::Result<YabaiMessage> {
    let command = get_config().map(|config| config.yabai_path).unwrap_or("yabai".to_string());
    let message = self.message.clone().map(|m| m.into()).ok_or_eyre("no command set")?;
    Ok(YabaiMessage { command, message, is_write: true })
  }
}

impl YabaiMessage {
  pub fn query() -> YabaiMessageBuilder<(), YabaiQueryCommandType> { YabaiMessageBuilder::default() }

  pub fn config() -> YabaiMessageBuilder<(), YabaiConfigCommandType> { YabaiMessageBuilder::default() }

  pub fn current_window() -> YabaiMessageBuilder<YabaiWindowSelector, YabaiWindowCommandType> {
    YabaiMessageBuilder::default()
  }

  pub fn window<T: Into<YabaiWindowSelector>>(
    window: T,
  ) -> YabaiMessageBuilder<YabaiWindowSelector, YabaiWindowCommandType> {
    YabaiMessageBuilder { selector: Some(window.into()), ..Default::default() }
  }

  pub fn current_display() -> YabaiMessageBuilder<YabaiDisplaySelector, YabaiDisplayCommandType> {
    YabaiMessageBuilder::default()
  }

  pub fn display<T: Into<YabaiDisplaySelector>>(
    display: T,
  ) -> YabaiMessageBuilder<YabaiDisplaySelector, YabaiDisplayCommandType> {
    YabaiMessageBuilder { selector: Some(display.into()), ..Default::default() }
  }

  pub fn is_write(&self) -> bool { self.is_write }
}

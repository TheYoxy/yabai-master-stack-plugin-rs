use crate::yabai::command::{
  config_command_type::YabaiConfigCommandType, display_command_type::YabaiDisplayCommandType,
  display_selector::YabaiDisplaySelector, message_type::YabaiMessageType, query_command_type::YabaiQueryCommandType,
  window_command_type::YabaiWindowCommandType, window_selector::YabaiWindowSelector,
};

#[derive(Debug, Clone)]
pub struct YabaiMessage {
  /// The yabai path to use.
  pub(super) command: String,
  /// The message sent to yabai.
  pub(super) message: YabaiMessageType,
}

#[derive(Debug, Clone)]
pub struct YabaiMessageBuilder<Selector, Message> {
  pub(super) selector: Option<Selector>,
  pub(super) message: Option<Message>,
}
impl<Selector, Message> Default for YabaiMessageBuilder<Selector, Message> {
  fn default() -> Self { Self { selector: None, message: None } }
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
}

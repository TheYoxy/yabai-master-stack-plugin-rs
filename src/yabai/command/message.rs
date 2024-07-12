use color_eyre::eyre::OptionExt;

use crate::yabai::{
  command::{
    display_selector::YabaiDisplaySelector, message_type::YabaiMessageType, space_selector::YabaiSpaceSelector,
    window_command_type::YabaiWindowCommandType, window_selector::YabaiWindowSelector,
  },
  config::get_config,
};

#[derive(Debug, Clone)]
pub struct YabaiMessage {
  pub(super) command: String,
  pub(super) message: YabaiMessageType,
}

#[derive(Debug, Clone)]
pub struct YabaiMessageBuilder<T> {
  window: Option<YabaiWindowSelector>,
  message: Option<T>,
}
impl<T> Default for YabaiMessageBuilder<T> {
  fn default() -> Self { Self { window: None, message: None } }
}

impl YabaiMessage {
  pub fn current_window() -> YabaiMessageBuilder<YabaiWindowCommandType> { YabaiMessageBuilder::default() }

  pub fn window<T: Into<Option<YabaiWindowSelector>>>(window: T) -> YabaiMessageBuilder<YabaiWindowCommandType> {
    YabaiMessageBuilder { window: window.into(), ..Default::default() }
  }
}

impl YabaiMessageBuilder<YabaiWindowCommandType> {
  pub fn build(&self) -> color_eyre::Result<YabaiMessage> {
    let command = get_config().map(|config| config.yabai_path).unwrap_or("yabai".to_string());
    let message = self.message.as_ref().ok_or_eyre("no command set")?.clone();
    Ok(YabaiMessage { command, message: YabaiMessageType::Window(self.window.clone(), message) })
  }

  pub fn focus<T: Into<Option<YabaiWindowSelector>>>(&mut self, selector: T) -> color_eyre::Result<YabaiMessage> {
    self.message = Some(YabaiWindowCommandType::Focus(selector.into()));
    self.build()
  }

  pub fn close<T: Into<Option<YabaiWindowSelector>>>(&mut self, selector: T) -> color_eyre::Result<YabaiMessage> {
    self.message = Some(YabaiWindowCommandType::Close(selector.into()));
    self.build()
  }

  pub fn minimize<T: Into<Option<YabaiWindowSelector>>>(&mut self, selector: T) -> color_eyre::Result<YabaiMessage> {
    self.message = Some(YabaiWindowCommandType::Minimize(selector.into()));
    self.build()
  }

  pub fn deminimize<T: Into<YabaiWindowSelector>>(&mut self, selector: T) -> color_eyre::Result<YabaiMessage> {
    self.message = Some(YabaiWindowCommandType::Deminimize(selector.into()));
    self.build()
  }

  pub fn display<T: Into<YabaiDisplaySelector>>(&mut self, selector: T) -> color_eyre::Result<YabaiMessage> {
    self.message = Some(YabaiWindowCommandType::Display(selector.into()));
    self.build()
  }

  pub fn space<T: Into<YabaiSpaceSelector>>(&mut self, selector: T) -> color_eyre::Result<YabaiMessage> {
    self.message = Some(YabaiWindowCommandType::Space(selector.into()));
    self.build()
  }

  pub fn swap<T: Into<YabaiWindowSelector>>(&mut self, selector: T) -> color_eyre::Result<YabaiMessage> {
    self.message = Some(YabaiWindowCommandType::Swap(selector.into()));
    self.build()
  }

  pub fn warp<T: Into<YabaiWindowSelector>>(&mut self, selector: T) -> color_eyre::Result<YabaiMessage> {
    self.message = Some(YabaiWindowCommandType::Warp(selector.into()));
    self.build()
  }

  pub fn stack<T: Into<YabaiWindowSelector>>(&mut self, selector: T) -> color_eyre::Result<YabaiMessage> {
    self.message = Some(YabaiWindowCommandType::Stack(selector.into()));
    self.build()
  }
}

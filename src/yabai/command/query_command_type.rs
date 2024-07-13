use color_eyre::eyre::OptionExt;
use log::debug;

use crate::yabai::{
  command::{
    display_selector::YabaiDisplaySelector,
    message::{YabaiMessage, YabaiMessageBuilder},
    message_type::YabaiMessageType,
    space_selector::YabaiSpaceSelector,
    to_argument::ToArgument,
    to_command::Runnable,
    window_selector::YabaiWindowSelector,
  },
  config::get_config,
  display::Display,
  spaces::Space,
  window::Window,
};

#[derive(Debug, Clone)]
pub enum YabaiQueryCommandType {
  Displays,
  Display(Option<YabaiDisplaySelector>),
  Spaces,
  Space(Option<YabaiSpaceSelector>),
  Windows,
  Window(Option<YabaiWindowSelector>),
}

impl ToArgument for YabaiQueryCommandType {
  fn to_argument(&self) -> String {
    match self {
      YabaiQueryCommandType::Displays => "--displays".into(),
      YabaiQueryCommandType::Display(Some(selector)) => format!("--displays --display {}", selector.to_argument()),
      YabaiQueryCommandType::Display(None) => "--displays --display".into(),
      YabaiQueryCommandType::Spaces => "--spaces".into(),
      YabaiQueryCommandType::Space(Some(selector)) => format!("--spaces --space {}", selector.to_argument()),
      YabaiQueryCommandType::Space(None) => "--spaces --space".into(),
      YabaiQueryCommandType::Windows => "--windows".into(),
      YabaiQueryCommandType::Window(Some(selector)) => format!("--windows --window {}", selector.to_argument()),
      YabaiQueryCommandType::Window(None) => "--windows --window".into(),
    }
  }
}

impl YabaiMessageBuilder<(), YabaiQueryCommandType> {
  /// Build the YabaiMessage from the builder.
  fn build(&self) -> color_eyre::Result<YabaiMessage> {
    debug!("creating yabai message for config");
    let command = get_config().map(|config| config.yabai_path).unwrap_or("yabai".to_string());
    let message = self.message.as_ref().ok_or_eyre("no command set")?.clone();
    debug!("command: {}", command);
    Ok(YabaiMessage { command, message: YabaiMessageType::Query(message) })
  }

  pub fn displays(&mut self) -> color_eyre::Result<Vec<Display>> {
    self.message = Some(YabaiQueryCommandType::Displays);
    let output = self.build()?.run()?;
    serde_json::from_slice(&output.stdout).map_err(|e| e.into())
  }

  pub fn current_display(&mut self) -> color_eyre::Result<Display> {
    self.message = Some(YabaiQueryCommandType::Display(None));
    let output = self.build()?.run()?;
    serde_json::from_slice(&output.stdout).map_err(|e| e.into())
  }

  pub fn display<T: Into<YabaiDisplaySelector>>(&mut self, display: T) -> color_eyre::Result<Display> {
    self.message = Some(YabaiQueryCommandType::Display(Some(display.into())));
    let output = self.build()?.run()?;
    serde_json::from_slice(&output.stdout).map_err(|e| e.into())
  }

  pub fn spaces(&mut self) -> color_eyre::Result<Vec<Space>> {
    self.message = Some(YabaiQueryCommandType::Spaces);
    let output = self.build()?.run()?;
    serde_json::from_slice(&output.stdout).map_err(|e| e.into())
  }

  pub fn current_space(&mut self) -> color_eyre::Result<Space> {
    self.message = Some(YabaiQueryCommandType::Space(None));
    let output = self.build()?.run()?;
    serde_json::from_slice(&output.stdout).map_err(|e| e.into())
  }

  pub fn space<T: Into<YabaiSpaceSelector>>(&mut self, space: T) -> color_eyre::Result<Space> {
    self.message = Some(YabaiQueryCommandType::Space(Some(space.into())));
    let output = self.build()?.run()?;
    serde_json::from_slice(&output.stdout).map_err(|e| e.into())
  }

  pub fn windows(&mut self) -> color_eyre::Result<Vec<Window>> {
    self.message = Some(YabaiQueryCommandType::Windows);
    let output = self.build()?.run()?;
    serde_json::from_slice(&output.stdout).map_err(|e| e.into())
  }

  pub fn current_window(&mut self) -> color_eyre::Result<Window> {
    self.message = Some(YabaiQueryCommandType::Window(None));
    let output = self.build()?.run()?;
    serde_json::from_slice(&output.stdout).map_err(|e| e.into())
  }

  pub fn window<T: Into<YabaiWindowSelector>>(&mut self, window: T) -> color_eyre::Result<Window> {
    self.message = Some(YabaiQueryCommandType::Window(Some(window.into())));
    let output = self.build()?.run()?;
    serde_json::from_slice(&output.stdout).map_err(|e| e.into())
  }
}

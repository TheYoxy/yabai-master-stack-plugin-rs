use color_eyre::eyre::OptionExt;

use crate::yabai::{
  command::{
    direction_selector::YabaiDirectionSelector,
    display_selector::YabaiDisplaySelector,
    message::{YabaiMessage, YabaiMessageBuilder},
    message_type::YabaiMessageType,
    space_selector::YabaiSpaceSelector,
    to_argument::ToArgument,
    toggle_selector::YabaiToggleSelector,
    window_selector::YabaiWindowSelector,
  },
  config::get_config,
};

#[derive(Debug, Clone)]
pub enum YabaiWindowCommandType {
  /// Focus the given window.
  /// If none specified, focus the selected window instead.
  Focus(Option<YabaiWindowSelector>),
  /// Close the given window.
  /// If none specified, close the selected window instead.
  /// Only works on windows that provide a close button in its title bar.
  Close(Option<YabaiWindowSelector>),
  /// Minimize the given window.
  /// If none specified, minimize the selected window instead.
  /// Only works on windows that provide a minimize button in its title bar.
  Minimize(Option<YabaiWindowSelector>),
  /// Restore the given window if it is minimized.
  /// The window will only get focus if the owning application has focus.
  /// Note that you can also --focus a minimized window to restore it as the focused window.
  Deminimize(YabaiWindowSelector),
  /// Send the selected window to the given display.
  Display(YabaiDisplaySelector),
  /// Send the selected window to the given space.
  Space(YabaiSpaceSelector),
  /// Swap position of the selected window and the given window.
  Swap(YabaiWindowSelector),
  /// Re-insert the selected window, splitting the given window.
  Warp(YabaiWindowSelector),
  /// Stack the given window on top of the selected window.
  /// Any kind of warp operation performed on a stacked window will unstack it.
  Stack(YabaiWindowSelector),
  Insert(YabaiDirectionSelector),
  // Grid()
  /// Toggle the given property of the selected window.
  /// The following properties requires System Integrity Protection to be partially disabled: sticky, pip, shadow, LABEL
  Toggle(YabaiToggleSelector),
}

impl YabaiMessageBuilder<YabaiWindowSelector, YabaiWindowCommandType> {
  /// Build the YabaiMessage from the builder.
  fn build(&self) -> color_eyre::Result<YabaiMessage> {
    let command = get_config().map(|config| config.yabai_path).unwrap_or("yabai".to_string());
    let message = self.message.as_ref().ok_or_eyre("no command set")?.clone();
    Ok(YabaiMessage { command, message: YabaiMessageType::Window(self.selector.clone(), message) })
  }

  /// Focus the given window.
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

  pub fn insert<T: Into<YabaiDirectionSelector>>(&mut self, selector: T) -> color_eyre::Result<YabaiMessage> {
    self.message = Some(YabaiWindowCommandType::Insert(selector.into()));
    self.build()
  }

  pub fn toggle<T: Into<YabaiToggleSelector>>(&mut self, selector: T) -> color_eyre::Result<YabaiMessage> {
    self.message = Some(YabaiWindowCommandType::Toggle(selector.into()));
    self.build()
  }
}

impl ToArgument for YabaiWindowCommandType {
  fn to_argument(&self) -> String {
    match self {
      YabaiWindowCommandType::Focus(selector) => {
        format!("--focus {}", selector.as_ref().map(|a| a.to_argument()).unwrap_or_default())
      },
      YabaiWindowCommandType::Close(selector) => {
        format!("--close {}", selector.as_ref().map(|a| a.to_argument()).unwrap_or_default())
      },
      YabaiWindowCommandType::Minimize(selector) => {
        format!("--minimize {}", selector.as_ref().map(|a| a.to_argument()).unwrap_or_default())
      },
      YabaiWindowCommandType::Deminimize(selector) => format!("--deminimize {}", selector.to_argument()),
      YabaiWindowCommandType::Display(selector) => format!("--display {}", selector.to_argument()),
      YabaiWindowCommandType::Space(selector) => format!("--space {}", selector.to_argument()),
      YabaiWindowCommandType::Swap(selector) => format!("--swap {}", selector.to_argument()),
      YabaiWindowCommandType::Warp(selector) => format!("--warp {}", selector.to_argument()),
      YabaiWindowCommandType::Stack(selector) => format!("--stack {}", selector.to_argument()),
      YabaiWindowCommandType::Insert(selector) => {
        format!("--insert {}", selector.to_argument())
      },
      YabaiWindowCommandType::Toggle(selector) => {
        format!("--toggle {}", selector.to_argument())
      },
    }
  }
}

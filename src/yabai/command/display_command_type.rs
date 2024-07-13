use color_eyre::eyre::OptionExt;

use crate::yabai::{
  command::{
    display_selector::YabaiDisplaySelector,
    message::{YabaiMessage, YabaiMessageBuilder},
    message_type::YabaiMessageType,
    space_selector::YabaiSpaceSelector,
    to_argument::ToArgument,
  },
  config::get_config,
};

#[derive(Debug, Clone)]
pub enum YabaiDisplayCommandType {
  Focus(YabaiDisplaySelector),
  Space(YabaiSpaceSelector),
  Label(String),
}

impl YabaiMessageBuilder<YabaiDisplaySelector, YabaiDisplayCommandType> {
  /// Build the YabaiMessage from the builder.
  fn build(&self) -> color_eyre::Result<YabaiMessage> {
    let command = get_config().map(|config| config.yabai_path).unwrap_or("yabai".to_string());
    let message = self.message.as_ref().ok_or_eyre("no command set")?.clone();
    Ok(YabaiMessage { command, message: YabaiMessageType::Display(self.selector.clone(), message), is_write: true })
  }

  pub fn focus<T: Into<YabaiDisplaySelector>>(&mut self, selector: T) -> color_eyre::Result<YabaiMessage> {
    self.message = Some(YabaiDisplayCommandType::Focus(selector.into()));
    self.build()
  }

  pub fn space<T: Into<YabaiSpaceSelector>>(&mut self, selector: T) -> color_eyre::Result<YabaiMessage> {
    self.message = Some(YabaiDisplayCommandType::Space(selector.into()));
    self.build()
  }

  pub fn label<T: Into<String>>(&mut self, selector: T) -> color_eyre::Result<YabaiMessage> {
    self.message = Some(YabaiDisplayCommandType::Label(selector.into()));
    self.build()
  }
}
impl ToArgument for YabaiDisplayCommandType {
  fn to_argument(&self) -> String {
    match self {
      YabaiDisplayCommandType::Focus(selector) => {
        format!("--focus {}", selector.to_argument())
      },
      YabaiDisplayCommandType::Space(selector) => {
        format!("--space {}", selector.to_argument())
      },
      YabaiDisplayCommandType::Label(label) => {
        format!("--label {}", label)
      },
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_focus() {
    let message = YabaiMessage::current_display().focus(1).unwrap();
    assert_eq!(message.message.to_argument(), "display --focus 1");
  }

  #[test]
  fn test_space() {
    let message = YabaiMessage::current_display().space(1).unwrap();
    assert_eq!(message.message.to_argument(), "display --space 1");
  }

  #[test]
  fn test_label() {
    let message = YabaiMessage::current_display().label("test").unwrap();
    assert_eq!(message.message.to_argument(), "display --label test");
  }
}

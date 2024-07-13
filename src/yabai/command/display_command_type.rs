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
    Ok(YabaiMessage { command, message: YabaiMessageType::Display(self.selector.clone(), message) })
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

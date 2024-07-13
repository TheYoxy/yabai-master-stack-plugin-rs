use color_eyre::eyre::Context;

use crate::yabai::command::{message::YabaiMessageBuilder, to_argument::ToArgument, to_command::Runnable};

#[derive(Debug, Clone)]
pub enum YabaiConfigCommandType {
  LeftPadding,
}

impl YabaiMessageBuilder<(), YabaiConfigCommandType> {
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

use log::debug;

use crate::yabai::command::{
  config_command_type::YabaiConfigCommandType, display_command_type::YabaiDisplayCommandType,
  display_selector::YabaiDisplaySelector, query_command_type::YabaiQueryCommandType, to_argument::ToArgument,
  window_command_type::YabaiWindowCommandType, window_selector::YabaiWindowSelector,
};

#[derive(Debug, Clone)]
pub enum YabaiMessageType {
  Window(Option<YabaiWindowSelector>, YabaiWindowCommandType),
  Display(Option<YabaiDisplaySelector>, YabaiDisplayCommandType),
  Config(YabaiConfigCommandType),
  Query(YabaiQueryCommandType),
}
impl ToArgument for YabaiMessageType {
  fn to_argument(&self) -> String {
    debug!("YabaiMessageType::to_argument: {:?}", self);
    match self {
      YabaiMessageType::Window(Some(window), selector) => {
        format!("window {} {}", window.to_argument(), selector.to_argument())
      },
      YabaiMessageType::Window(None, selector) => format!("window {}", selector.to_argument()),
      YabaiMessageType::Display(Some(display), selector) => {
        format!("display {} {}", display.to_argument(), selector.to_argument())
      },
      YabaiMessageType::Display(None, selector) => format!("display {}", selector.to_argument()),
      YabaiMessageType::Config(config) => format!("config {}", config.to_argument()),
      YabaiMessageType::Query(query) => format!("query {}", query.to_argument()),
    }
  }
}

impl From<YabaiQueryCommandType> for YabaiMessageType {
  fn from(command: YabaiQueryCommandType) -> Self { YabaiMessageType::Query(command) }
}
impl From<YabaiConfigCommandType> for YabaiMessageType {
  fn from(command: YabaiConfigCommandType) -> Self { YabaiMessageType::Config(command) }
}

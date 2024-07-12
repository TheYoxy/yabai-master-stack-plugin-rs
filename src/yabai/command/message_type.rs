use crate::yabai::command::{window_command_type::YabaiWindowCommandType, window_selector::YabaiWindowSelector};

#[derive(Debug, Clone)]
pub enum YabaiMessageType {
  Window(Option<YabaiWindowSelector>, YabaiWindowCommandType),
  Display,
}

use crate::yabai::command::{
  direction_selector::YabaiDirectionSelector, display_selector::YabaiDisplaySelector, message_type::YabaiMessageType,
  space_selector::YabaiSpaceSelector, stack_selector::YabaiStackSelector, window_command_type::YabaiWindowCommandType,
  window_selector::YabaiWindowSelector,
};

pub(super) trait ToArgument {
  fn to_argument(&self) -> String;
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
    }
  }
}
impl ToArgument for YabaiStackSelector {
  fn to_argument(&self) -> String {
    match self {
      YabaiStackSelector::Prev => "stack.prev".into(),
      YabaiStackSelector::Next => "stack.next".into(),
      YabaiStackSelector::First => "stack.first".into(),
      YabaiStackSelector::Last => "stack.last".into(),
      YabaiStackSelector::Recent => "stack.recent".into(),
      YabaiStackSelector::Index(index) => format!("stack.{}", index),
    }
  }
}

impl ToArgument for YabaiDirectionSelector {
  fn to_argument(&self) -> String {
    match self {
      YabaiDirectionSelector::North => "north",
      YabaiDirectionSelector::East => "east",
      YabaiDirectionSelector::South => "south",
      YabaiDirectionSelector::West => "west",
    }
    .into()
  }
}

impl ToArgument for YabaiMessageType {
  fn to_argument(&self) -> String {
    match self {
      YabaiMessageType::Window(Some(window), selector) => {
        format!("window {} {}", window.to_argument(), selector.to_argument())
      },
      YabaiMessageType::Window(None, selector) => format!("window {}", selector.to_argument()),
      YabaiMessageType::Display => todo!(),
    }
  }
}
impl ToArgument for YabaiSpaceSelector {
  fn to_argument(&self) -> String {
    match self {
      YabaiSpaceSelector::Prev => "prev".into(),
      YabaiSpaceSelector::Next => "next".into(),
      YabaiSpaceSelector::First => "first".into(),
      YabaiSpaceSelector::Last => "last".into(),
      YabaiSpaceSelector::Recent => "recent".into(),
      YabaiSpaceSelector::Mouse => "mouse".into(),
      YabaiSpaceSelector::MissionControlIndex(index) => index.to_string(),
      YabaiSpaceSelector::Label(label) => label.into(),
    }
  }
}
impl ToArgument for YabaiWindowSelector {
  fn to_argument(&self) -> String {
    match self {
      YabaiWindowSelector::Prev => "prev".into(),
      YabaiWindowSelector::Next => "next".into(),
      YabaiWindowSelector::First => "first".into(),
      YabaiWindowSelector::Last => "last".into(),
      YabaiWindowSelector::Recent => "recent".into(),
      YabaiWindowSelector::Mouse => "mouse".into(),
      YabaiWindowSelector::Largest => "largest".into(),
      YabaiWindowSelector::Smallest => "smallest".into(),
      YabaiWindowSelector::Sibling => "sibling".into(),
      YabaiWindowSelector::FirstNephew => "first_nephew".into(),
      YabaiWindowSelector::SecondNephew => "second_nephew".into(),
      YabaiWindowSelector::Uncle => "uncle".into(),
      YabaiWindowSelector::FirstCousin => "first_cousin".into(),
      YabaiWindowSelector::SecondCousin => "second_cousin".into(),
      YabaiWindowSelector::StackSelector(selector) => selector.to_argument(),
      YabaiWindowSelector::DirectionSelector(selector) => selector.to_argument(),
      YabaiWindowSelector::Id(id) => id.to_string(),
    }
  }
}
impl ToArgument for YabaiDisplaySelector {
  fn to_argument(&self) -> String {
    match self {
      YabaiDisplaySelector::Prev => "prev".into(),
      YabaiDisplaySelector::Next => "next".into(),
      YabaiDisplaySelector::First => "first".into(),
      YabaiDisplaySelector::Last => "last".into(),
      YabaiDisplaySelector::Recent => "recent".into(),
      YabaiDisplaySelector::Mouse => "mouse".into(),
      YabaiDisplaySelector::DirectionSelector(selector) => selector.to_argument(),
      YabaiDisplaySelector::ArrangementIndex(index) => index.to_string(),
      YabaiDisplaySelector::Label(label) => label.into(),
    }
  }
}

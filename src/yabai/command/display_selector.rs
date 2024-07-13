use crate::yabai::{
  command::{direction_selector::YabaiDirectionSelector, to_argument::ToArgument},
  config::MasterPosition,
  display::Display,
};

// DISPLAY_SEL := prev | next | first | last | recent | mouse | DIR_SEL | <arrangement index (1-based)> | LABEL
#[derive(Debug, Clone)]
pub enum YabaiDisplaySelector {
  /// prev
  Prev,
  /// next
  Next,
  /// first
  First,
  /// last
  Last,
  /// recent
  Recent,
  /// mouse
  Mouse,
  /// DIR_SEL
  DirectionSelector(YabaiDirectionSelector),
  /// <arrangement index (1-based)>
  ArrangementIndex(usize),
  /// LABEL
  Label(String),
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
impl From<MasterPosition> for YabaiDisplaySelector {
  fn from(master_position: MasterPosition) -> Self { YabaiDisplaySelector::DirectionSelector(master_position.into()) }
}
impl From<usize> for YabaiDisplaySelector {
  fn from(index: usize) -> Self { YabaiDisplaySelector::ArrangementIndex(index) }
}
impl From<Display> for YabaiDisplaySelector {
  fn from(value: Display) -> Self { YabaiDisplaySelector::ArrangementIndex(value.id) }
}
impl From<&Display> for YabaiDisplaySelector {
  fn from(value: &Display) -> Self { YabaiDisplaySelector::ArrangementIndex(value.id) }
}

use crate::yabai::{
  command::{direction_selector::YabaiDirectionSelector, stack_selector::YabaiStackSelector, to_argument::ToArgument},
  config::MasterPosition,
  window::Window,
};

// WINDOW_SEL  := prev | next | first | last | recent | mouse | largest | smallest | sibling | first_nephew | second_nephew | uncle | first_cousin | second_cousin | STACK_SEL | DIR_SEL | <window id>
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum YabaiWindowSelector {
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
  /// largest
  Largest,
  /// smallest
  Smallest,
  /// sibling
  Sibling,
  /// first_nephew
  FirstNephew,
  /// second_nephew
  SecondNephew,
  /// uncle
  Uncle,
  /// first_cousin
  FirstCousin,
  /// second_cousin
  SecondCousin,

  /// STACK_SEL
  StackSelector(YabaiStackSelector),
  /// DIR_SEL
  DirectionSelector(YabaiDirectionSelector),
  /// <window id>
  Id(usize),
}

impl From<usize> for YabaiWindowSelector {
  fn from(value: usize) -> Self { Self::Id(value) }
}
impl From<Window> for YabaiWindowSelector {
  fn from(value: Window) -> Self { Self::Id(value.id) }
}
impl From<&Window> for YabaiWindowSelector {
  fn from(value: &Window) -> Self { Self::Id(value.id) }
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
impl From<YabaiDirectionSelector> for YabaiWindowSelector {
  fn from(selector: YabaiDirectionSelector) -> Self { YabaiWindowSelector::DirectionSelector(selector) }
}
impl From<YabaiDirectionSelector> for Option<YabaiWindowSelector> {
  fn from(selector: YabaiDirectionSelector) -> Self { Some(YabaiWindowSelector::DirectionSelector(selector)) }
}
impl From<MasterPosition> for YabaiWindowSelector {
  fn from(value: MasterPosition) -> Self { YabaiWindowSelector::DirectionSelector(value.into()) }
}

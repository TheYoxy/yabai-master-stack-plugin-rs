use crate::yabai::command::{direction_selector::YabaiDirectionSelector, stack_selector::YabaiStackSelector};

// WINDOW_SEL  := prev | next | first | last | recent | mouse | largest | smallest | sibling | first_nephew | second_nephew | uncle | first_cousin | second_cousin | STACK_SEL | DIR_SEL | <window id>
#[derive(Debug, Clone)]
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

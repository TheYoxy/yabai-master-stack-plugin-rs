use crate::yabai::command::direction_selector::YabaiDirectionSelector;

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

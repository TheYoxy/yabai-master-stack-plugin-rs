use crate::yabai::command::to_argument::ToArgument;

// STACK_SEL   := stack.prev | stack.next | stack.first | stack.last | stack.recent | stack.<index (1-based)>
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum YabaiStackSelector {
  /// stack.prev
  Prev,
  /// stack.next
  Next,
  /// stack.first
  First,
  /// stack.last
  Last,
  /// stack.recent
  Recent,
  /// stack.<index (1-based)>
  Index(usize),
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

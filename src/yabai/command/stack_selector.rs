// STACK_SEL   := stack.prev | stack.next | stack.first | stack.last | stack.recent | stack.<index (1-based)>
#[derive(Clone, Debug)]
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

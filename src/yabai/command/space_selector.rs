// SPACE_SEL   := prev | next | first | last | recent | mouse | <mission-control index (1-based)> | LABEL
#[derive(Clone, Debug)]
pub enum YabaiSpaceSelector {
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
  /// <mission-control index (1-based)>
  MissionControlIndex(usize),
  /// LABEL
  Label(String),
}

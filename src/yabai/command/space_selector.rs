use crate::yabai::command::to_argument::ToArgument;

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

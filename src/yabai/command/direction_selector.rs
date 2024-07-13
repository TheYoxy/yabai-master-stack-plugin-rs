use crate::yabai::{command::to_argument::ToArgument, config::MasterPosition};

#[derive(Clone, Debug, Eq, PartialEq)]
// DIR_SEL     := north | east | south | west
pub enum YabaiDirectionSelector {
  North,
  East,
  South,
  West,
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

impl From<MasterPosition> for YabaiDirectionSelector {
  fn from(value: MasterPosition) -> Self {
    match value {
      MasterPosition::Left => YabaiDirectionSelector::West,
      MasterPosition::Right => YabaiDirectionSelector::East,
    }
  }
}

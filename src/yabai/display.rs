use std::fmt::Formatter;

use color_eyre::owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::spaces::SpaceId;
use crate::yabai::frame::Frame;

pub type DisplayId = usize;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Display {
  pub(crate) id: DisplayId,
  pub(crate) uuid: Uuid,
  pub(crate) index: usize,
  pub(crate) frame: Frame,
  pub(crate) spaces: Vec<SpaceId>,
}

impl std::fmt::Display for Display {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "Display {} ({} {})", self.index.blue(), self.id.blue(), self.frame)
  }
}

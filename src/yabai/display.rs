use std::fmt::Formatter;

use color_eyre::owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::spaces::SpaceId;
use crate::yabai::commands::{get_yabai_command, RunCommand};

pub type DisplayId = usize;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Frame {
  pub(crate) x: f64,
  pub(crate) y: f64,
  pub(crate) w: f64,
  pub(crate) h: f64,
}

impl std::fmt::Display for Frame {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "Frame (x: {} y: {} w: {} h: {})", self.x.blue(), self.y.blue(), self.w.blue(), self.h.blue())
  }
}

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

pub fn get_displays() -> color_eyre::Result<Vec<Display>> {
  let output = get_yabai_command()?.args(["-m", "query", "--displays"]).run_command_with_output()?;
  serde_json::from_slice(&output.stdout).map_err(|e| e.into())
}

pub fn get_focused_display() -> color_eyre::Result<Display> {
  let output = get_yabai_command()?.args(["-m", "query", "--displays", "--display"]).run_command_with_output()?;
  serde_json::from_slice(&output.stdout).map_err(|e| e.into())
}

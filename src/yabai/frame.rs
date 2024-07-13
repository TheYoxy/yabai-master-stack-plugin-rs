use std::fmt::Formatter;

use color_eyre::owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};

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

use std::fmt::{Display, Formatter};

use color_eyre::owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};

use crate::yabai::frame::Frame;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum SplitType {
  Horizontal,
  Vertical,
  #[serde(untagged)]
  Unknown(String),
}

pub type WindowId = usize;
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Window {
  pub(crate) id: WindowId,
  pub(crate) pid: usize,
  pub(crate) app: String,
  pub(crate) title: String,
  pub(crate) frame: Frame,
  pub(crate) role: String,
  pub(crate) subrole: String,
  pub(crate) display: usize,
  pub(crate) space: usize,
  pub(crate) level: usize,
  pub(crate) sub_level: usize,
  pub(crate) layer: String,
  pub(crate) sub_layer: String,
  pub(crate) opacity: f64,
  pub(crate) split_type: SplitType,
  pub(crate) split_child: String,
  pub(crate) stack_index: usize,
  pub(crate) can_move: bool,
  pub(crate) can_resize: bool,
  pub(crate) has_focus: bool,
  pub(crate) has_shadow: bool,
  pub(crate) has_parent_zoom: bool,
  pub(crate) has_fullscreen_zoom: bool,
  pub(crate) has_ax_reference: bool,
  pub(crate) is_native_fullscreen: bool,
  pub(crate) is_visible: bool,
  pub(crate) is_minimized: bool,
  pub(crate) is_hidden: bool,
  pub(crate) is_floating: bool,
  pub(crate) is_sticky: bool,
  pub(crate) is_grabbed: bool,
}

impl Display for Window {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { write!(f, "{} [{}]", self.app.blue(), self.id.yellow()) }
}

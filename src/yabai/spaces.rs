use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type WindowId = usize;
pub type SpaceId = usize;
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct Space {
  pub(crate) id: SpaceId,
  pub(crate) uuid: Uuid,
  pub(crate) index: usize,
  pub(crate) label: String,
  pub(crate) r#type: String,
  pub(crate) display: usize,
  pub(crate) windows: Vec<WindowId>,
  pub(crate) first_window: WindowId,
  pub(crate) last_window: WindowId,
  pub(crate) has_focus: bool,
  pub(crate) is_visible: usize,
  pub(crate) is_native_fullscreen: bool,
}

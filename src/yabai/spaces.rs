use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::command::get_yabai_command;

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

pub fn get_spaces() -> color_eyre::Result<Vec<Space>> {
    let mut command = get_yabai_command()?;
    let output = command.args(["-m", "query", "--spaces"]).output()?;
    serde_json::from_slice(&output.stdout).map_err(|e| e.into())
}

pub fn get_focused_space() -> color_eyre::Result<Space> {
    let mut command = get_yabai_command()?;
    let output = command
        .args(["-m", "query", "--spaces", "--space"])
        .output()?;
    serde_json::from_slice(&output.stdout).map_err(|e| e.into())
}

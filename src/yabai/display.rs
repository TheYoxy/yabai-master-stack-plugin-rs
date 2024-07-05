use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{command::get_yabai_command, spaces::SpaceId};

pub type DisplayId = usize;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Frame {
    pub(crate) x: f64,
    pub(crate) y: f64,
    pub(crate) w: f64,
    pub(crate) h: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Display {
    pub(crate) id: DisplayId,
    pub(crate) uuid: Uuid,
    pub(crate) index: usize,
    pub(crate) frame: Frame,
    pub(crate) spaces: Vec<SpaceId>,
}

pub fn get_focused_display() -> color_eyre::Result<Display> {
    let mut command = get_yabai_command()?;
    let output = command
        .args(["-m", "query", "--displays", "--display"])
        .output()?;
    serde_json::from_slice(&output.stdout).map_err(|e| e.into())
}

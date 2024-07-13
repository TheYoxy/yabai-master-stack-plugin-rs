use color_eyre::owo_colors::OwoColorize;
use log::trace;

use crate::{
  window_manager::WindowsManager,
  yabai::{command::message::YabaiMessage, display::Display, spaces::Space, state::State},
};

pub struct InitializedWindowsManager {
  pub wm: WindowsManager,
  pub state: State,
  pub display: Display,
  pub space: Space,
}

pub(super) fn create_initialized_windows_manager() -> color_eyre::Result<InitializedWindowsManager> {
  trace!("Initializing windows manager");
  let mut state = State::read_state()?;
  let display = YabaiMessage::query().current_display()?;
  trace!("Focused display: {:?}", display);
  let space = YabaiMessage::query().current_space()?;
  trace!("Focused space: {:?}", space);
  let space_state = state.get_space(&space)?;
  trace!("Space state: {} for {}", space_state.blue(), space.id.blue());
  let mut wm = WindowsManager::new(display.clone(), space.clone(), *space_state);
  wm.initialize()?;
  wm.validate_state(&mut state)?;

  Ok(InitializedWindowsManager { wm, state, display, space })
}

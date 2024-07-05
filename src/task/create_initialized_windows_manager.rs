use crate::yabai::display::{get_focused_display, Display};
use crate::yabai::spaces::{get_focused_space, Space};
use crate::yabai::state::{read_state, State, StateForSpace};
use crate::yabai::window_manager::ctor::WindowsManager;
use color_eyre::owo_colors::OwoColorize;
use log::trace;

pub(super) struct InitializedWindowsManager {
    pub(crate) wm: WindowsManager,
    pub(crate) state: State,
    pub(crate) display: Display,
    pub(crate) space: Space,
}

pub(super) fn create_initialized_windows_manager() -> color_eyre::Result<InitializedWindowsManager>
{
    trace!("Initializing windows manager");
    let mut state = read_state()?;
    let display = get_focused_display()?;
    trace!("Focused display: {:?}", display);
    let space = get_focused_space()?;
    trace!("Focused space: {:?}", space);
    let space_state = state.get_space(&space)?;
    trace!(
        "Space state: {} for {}",
        space_state.blue(),
        space.id.blue()
    );
    let mut wm = WindowsManager::new(display.clone(), space.clone(), *space_state);
    wm.initialize()?;
    wm.validate_state(&mut state)?;

    Ok(InitializedWindowsManager {
        wm,
        state,
        display,
        space,
    })
}
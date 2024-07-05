use log::trace;

use crate::task::create_initialized_windows_manager::InitializedWindowsManager;
use crate::yabai::state::StateForSpace;

pub(crate) fn on_yabai_start(iwm: &mut InitializedWindowsManager) -> color_eyre::Result<()> {
    trace!("Handling on yabai start event");
    let wm = &mut iwm.wm;

    let state = &iwm.state;
    let space = &iwm.space;
    let space_state = state.get_space(space)?;
    trace!("Updating windows...");
    wm.update_windows(*space_state)?;
    trace!("On yabai start event handled");

    Ok(())
}

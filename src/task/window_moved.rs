use crate::task::create_initialized_windows_manager::InitializedWindowsManager;
use crate::yabai::state::StateForSpace;
use log::trace;

pub fn window_moved(iwm: &mut InitializedWindowsManager) -> color_eyre::Result<()> {
    trace!("Handling window moved event");

    let wm = &mut iwm.wm;
    let state = &iwm.state;
    let space = &iwm.space;
    let space_state = state.get_space(space)?;
    trace!("Updating windows...");
    wm.update_windows(*space_state)?;
    trace!("Window moved event handled");

    Ok(())
}

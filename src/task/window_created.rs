use crate::task::create_initialized_windows_manager::InitializedWindowsManager;
use crate::yabai::config::get_config;
use crate::yabai::state::StateForSpace;
use crate::yabai::window_manager::layout_visibility::LayoutValidity;
use log::trace;

pub(crate) fn window_created(iwm: &mut InitializedWindowsManager) -> color_eyre::Result<()> {
    trace!("Handling window created event");

    let wm = &mut iwm.wm;
    let state = &iwm.state;
    let space = &iwm.space;
    let layout_validity = wm.is_valid_layout(None)?;

    match layout_validity {
        LayoutValidity::Valid => {
            trace!("Layout is valid, no changes were made.");
            Ok(())
        }
        LayoutValidity::Invalid(_) => {
            trace!("Window created event handled");
            let process_id: usize = std::env::var("YABAI_PROCESS_ID")?.parse()?;
            let window_id: usize = std::env::var("YABAI_WINDOW_ID")?.parse()?;
            let cur_num_master_windows = wm.get_master_windows()?.len();
            let window = wm.get_window_data(process_id, window_id)?;
            let space_state = state.get_space(space)?;
            let config = get_config()?;
            if config.move_new_windows_to_master {
                // If the master is full, move a window from master to stack
                if cur_num_master_windows >= *space_state {
                    let old_master_windows = wm.get_master_window()?;
                    if let Some(old_master_windows) = old_master_windows {
                        wm.move_window_to_stack(&old_master_windows)?;
                    }
                    wm.move_window_to_master(window)?;
                } else {
                    wm.move_window_to_master(window)?;
                }
            } else if cur_num_master_windows > 1 && cur_num_master_windows <= *space_state {
                trace!("Moving new window {window} to master");
                wm.move_window_to_master(window)?;
            } else {
                trace!("Moving new window {window} to stack");
                wm.move_window_to_stack(window)?;
            }
            wm.update_windows(*space_state)?;
            Ok(())
        }
    }
}

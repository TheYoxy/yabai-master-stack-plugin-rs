use log::debug;

use crate::yabai::{window::Window, window_manager::ctor::WindowsManager};

type Result<T> = color_eyre::Result<T>;

fn get_widest(windows: Vec<Window>) -> Option<Window> {
  let mut widest_stack_window: Option<Window> = None;
  debug!("Looking for widest stack window in {len} windows", len = windows.len());
  for window in windows {
    if let Some(ref current_window) = widest_stack_window {
      if window.frame.w > current_window.frame.w {
        widest_stack_window = Some(window);
      }
    } else {
      widest_stack_window = Some(window);
    }
  }

  widest_stack_window
}

impl WindowsManager {
  pub(super) fn get_widest_master_window(&self) -> Result<Option<Window>> {
    let master_windows = self.get_master_windows()?;

    Ok(get_widest(master_windows))
  }

  pub(super) fn get_widest_stack_window(&self) -> Option<Window> {
    let stack_windows = self.get_stack_windows();
    get_widest(stack_windows)
  }
}

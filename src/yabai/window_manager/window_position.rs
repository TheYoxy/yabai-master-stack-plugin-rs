use log::{debug, error, trace};

use crate::yabai::{window::Window, window_manager::WindowsManager};

type Result<T> = color_eyre::Result<T>;
impl WindowsManager {
  pub(super) fn get_bottom_window(&self, windows: Vec<Window>) -> Option<Window> {
    if windows.is_empty() {
      debug!("No windows provided to find bottom window");
      return None;
    }

    let mut bottom_window = windows.first().unwrap();

    for window in windows.iter() {
      if window.frame.y > bottom_window.frame.y {
        bottom_window = window;
      }
    }

    Some(bottom_window.clone())
  }

  pub(crate) fn is_top_window(&self, windows: Vec<Window>, focused_window: &Window) -> bool {
    let top_window = self.get_top_window(windows);
    if let Some(top_window) = top_window {
      top_window.id == focused_window.id
    } else {
      false
    }
  }

  pub(crate) fn is_bottom_window(&self, windows: Vec<Window>, focused_window: &Window) -> bool {
    let bottom_window = self.get_bottom_window(windows);
    if let Some(bottom_window) = bottom_window {
      bottom_window.id == focused_window.id
    } else {
      false
    }
  }

  pub(super) fn get_top_window(&self, windows: Vec<Window>) -> Option<Window> {
    if windows.is_empty() {
      debug!("No windows provided to find top window");
      return None;
    }

    let mut top_window = windows.first().unwrap();
    for window in windows.iter() {
      if window.frame.y < top_window.frame.y {
        top_window = window;
      }
    }

    Some(top_window.clone())
  }

  pub(super) fn get_top_left_window(&self) -> Option<Window> {
    trace!("Looking for top left window");
    if self.windows.is_empty() {
      trace!("No windows found");
      return None;
    }

    let mut left_windows = self
      .windows
      .clone()
      .into_iter()
      .filter(|window| {
        self
          .is_windows_touching_left_edge(window)
          .inspect_err(|err| error!("An error occurred while checking if windows touch: {err}"))
          .is_ok_and(|is_touching| is_touching)
      })
      .collect::<Vec<_>>();
    if left_windows.is_empty() {
      trace!("No left windows found");
      return None;
    }
    trace!("Found {len} left windows", len = left_windows.len());
    trace!("Sorting windows by y coordinate");
    left_windows.sort_by(|window1, window2| window1.frame.y.total_cmp(&window2.frame.y));
    trace!("Getting top left window");
    left_windows.first().cloned()
  }

  pub(super) fn get_top_right_window(&self) -> Option<Window> {
    if self.windows.is_empty() {
      return None;
    }

    let window = self.windows.first();
    if let Some(window) = window {
      let mut lowest_y_coordinate = window.frame.y;
      for window in self.windows.iter() {
        if window.frame.y < lowest_y_coordinate {
          lowest_y_coordinate = window.frame.y;
        }
      }

      let top_windows = self.windows.iter().filter(|window| window.frame.y == lowest_y_coordinate).collect::<Vec<_>>();
      let top_window = top_windows.first();
      if let Some(&top_window) = top_window {
        let mut top_window = top_window;
        for window in top_windows.iter() {
          if window.frame.x > top_window.frame.x {
            top_window = window
          }
        }
        Some(top_window.clone())
      } else {
        None
      }
    } else {
      None
    }
  }

  pub(super) fn is_middle_window(&self, window: &Window) -> bool {
    trace!("Checking if {window} is a middle window");

    !self
      .is_stack_window(window)
      .inspect_err(|err| error!("Error while getting is stack window: {err}"))
      .is_ok_and(|r| r)
      && !self
        .is_master_window(window)
        .inspect_err(|err| error!("Error while getting is stack window: {err}"))
        .is_ok_and(|r| r)
  }

  pub(super) fn get_middle_windows(&self) -> Vec<&Window> {
    debug!("Looking for middle windows");
    self.windows.iter().filter(|w| self.is_middle_window(w)).collect::<Vec<_>>()
  }
}

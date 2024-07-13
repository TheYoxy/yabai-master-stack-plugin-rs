use color_eyre::{eyre::bail, owo_colors::OwoColorize};
use log::{debug, info, trace, warn};

use crate::{
  window_manager::{layout_visibility::LayoutValidity, WindowsManager},
  yabai::{
    command::{message::YabaiMessage, toggle_selector::YabaiToggleSelector},
    window::{SplitType, Window},
  },
};

impl WindowsManager {
  pub fn update_windows(&mut self, target_num_master_windows: usize) -> color_eyre::Result<()> {
    info!("update_windows(Target master count = {target_num_master_windows})");
    if target_num_master_windows == 0 {
      bail!("Target number of master windows cannot be 0.");
    }

    #[cfg(debug_assertions)]
    {
      trace!("Windows: {len}", len = self.windows.len());
      for (idx, window) in self.windows.iter().enumerate() {
        trace!("Window {idx}: {window}");
      }
    }

    trace!("--------------------------");
    self.columnize_master_windows()?;
    trace!("--------------------------");
    self.columnize_stack_windows()?;
    debug!("Columnize stack windows");

    let layout_validity = self.is_valid_layout(Some(target_num_master_windows))?;
    let sort = |w1: &Window, w2: &Window| {
      if w1.frame.y == w2.frame.y {
        w1.frame.x.total_cmp(&w2.frame.x)
      } else {
        w1.frame.y.total_cmp(&w2.frame.y)
      }
    };
    debug!("Layout validity: {layout_validity:?}");
    match layout_validity {
      LayoutValidity::Valid => {
        info!("Layout is valid");
        Ok(())
      },
      LayoutValidity::Invalid(reason) => {
        info!("Invalid layout detected: {reason}. Updating windows...");
        trace!("Windows: {windows:?}", windows = self.windows);

        let num_windows = self.windows.len();
        if target_num_master_windows != num_windows && !self.does_stack_exists() {
          info!("Stack does not exists, creating it...");
          self.create_stack()?;
        }

        if target_num_master_windows == num_windows {
          for window in &self.windows {
            if window.split_type == SplitType::Vertical {
              let message = YabaiMessage::window(window).toggle(YabaiToggleSelector::Split)?;
              self.send_yabai_message(message)?;
            }
          }
        }

        if num_windows > 1 {
          let mut master_windows = self.get_master_windows()?;
          info!(
            "Master windows: {windows}",
            windows = master_windows.iter().map(|window| window.app.clone()).collect::<Vec<_>>().join(",")
          );

          let mut cur_num_master_windows = master_windows.len();
          if cur_num_master_windows > target_num_master_windows {
            info!("Too many master windows ({cur_num_master_windows}/{target_num_master_windows}).");
          }
          master_windows.sort_by(sort);

          let location = "stack".blue();
          while cur_num_master_windows > target_num_master_windows {
            let master_window = master_windows.pop();
            if let Some(master_window) = master_window {
              info!("Moving master window {master_window} to {location}.");
              self.move_window_to_stack(&master_window)?;
              info!("Moved window {master_window} to {location}.");
            }
            cur_num_master_windows -= 1;
            debug!("Master windows count: {cur_num_master_windows}");
          }

          let mut middle_windows = self.get_middle_windows();
          debug!("Middle windows: {middle_windows:?}",);
          let mut dead_lock = 0;

          // If there are windows that aren't touching either the left side or the right side
          // after the move, fill up master and then move the rest to stack
          while !middle_windows.is_empty() {
            let middle_window = middle_windows.first();
            if let Some(middle_window) = middle_window {
              info!("Middle window {middle_window} detected");
              if cur_num_master_windows < target_num_master_windows {
                let location = "master".blue();
                info!("Moving middle window {middle_window} to {location}.");
                self.move_window_to_master(middle_window)?;
                cur_num_master_windows += 1;
              } else {
                let location = "stack".blue();
                info!("Moving middle window {middle_window} to {location}.");
                self.move_window_to_stack(middle_window)?;
              }
              dead_lock += 1;
            } else {
              warn!("Unable to get middle window");
              dead_lock += 1;
            }
            #[cfg(not(debug_assertions))]
            let max = 10;
            #[cfg(debug_assertions)]
            let max = 1;
            if dead_lock > max {
              bail!("Dead lock detected while moving middle windows.")
            }
            debug!("-------------------------------------------------------------");
            middle_windows = self.get_middle_windows();
            debug!("-------------------------------------------------------------");
            debug!("Middle windows count: {len}", len = middle_windows.len());
          }

          let mut stack_windows = self.get_stack_windows();
          stack_windows.sort_by(sort);

          while cur_num_master_windows < target_num_master_windows {
            info!("Not enough master windows ({}/{})", cur_num_master_windows.blue(), target_num_master_windows.blue());
            let stack_window = stack_windows.pop().unwrap();
            info!("Moving stack window {stack_window} to master.",);
            self.move_window_to_master(&stack_window)?;
            cur_num_master_windows += 1;
          }

          let result = self.is_valid_layout(Some(target_num_master_windows))?;
          match result {
            LayoutValidity::Valid => info!("update_windows() was successful."),
            LayoutValidity::Invalid(reason) => {
              bail!("update_windows() ended with and invalid layout; reason {reason}")
            },
          }
          self.expected_current_num_master_windows = target_num_master_windows;
        } else {
          debug!("Only one window is open, no need to update windows.");
        }

        Ok(())
      },
    }
  }
}

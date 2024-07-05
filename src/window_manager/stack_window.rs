use log::{debug, error, trace};

use crate::{
  print_bool,
  window_manager::WindowsManager,
  yabai::{
    config::{get_config, MasterPosition},
    window::Window,
  },
};

type Result<T> = color_eyre::Result<T>;
impl WindowsManager {
  pub(crate) fn get_stack_windows(&self) -> Vec<Window> {
    debug!("Looking for stack windows in {len} windows", len = self.windows.len());

    let result: Vec<Window> = self
      .windows
      .clone()
      .into_iter()
      .filter(|window| self.is_stack_window(window).is_ok_and(|is_stack| is_stack))
      .collect();
    debug!("Found {len} stack windows", len = result.len());

    result
  }

  pub(crate) fn is_stack_window(&self, window: &Window) -> Result<bool> {
    trace!("Checking that {window} is not a stacked window");
    let config = get_config()?;
    let result = match config.master_position {
      MasterPosition::Left => {
        let dividing_line_x_coordinate = self.get_dividing_line_x_coordinate()?;
        trace!("Dividing line x coordinate: {dividing_line_x_coordinate} {x}", x = window.frame.x);
        window.frame.x == dividing_line_x_coordinate
      },
      MasterPosition::Right => {
        self
          .is_windows_touching_left_edge(window)
          .inspect_err(|err| error!("Error while checking if windows touch: {err}"))
          .is_ok_and(|is_touching| is_touching)
      },
    };

    trace!("{window} is a stacked window: {result}", result = print_bool!(result, "stacked", "unstacked"),);
    Ok(result)
  }

  pub(crate) fn get_top_stack_window(&self) -> Option<Window> { self.get_top_window(self.get_stack_windows()) }

  pub(crate) fn get_bottom_stack_window(&self) -> Option<Window> { self.get_bottom_window(self.get_stack_windows()) }
}

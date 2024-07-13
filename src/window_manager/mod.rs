use color_eyre::{eyre::eyre, owo_colors::OwoColorize};
use log::{debug, trace};

use crate::{
  print_bool,
  yabai::{
    command::message::YabaiMessage,
    display::Display,
    spaces::Space,
    state::{write_state, State, StateForSpace},
    window::Window,
  },
};

mod columnize;
mod layout;
pub mod layout_visibility;
mod master_window;
mod stack;
mod stack_window;
pub mod update_windows;
pub mod widest_window;
mod window_position;
pub mod windows;
pub mod yabai;

pub struct WindowsManager {
  pub(crate) display: Display,
  pub(crate) space: Space,
  pub(crate) expected_current_num_master_windows: usize,
  pub(crate) windows: Vec<Window>,
}

type Result<T> = color_eyre::Result<T>;

impl WindowsManager {
  pub fn new(display: Display, space: Space, expected_current_num_master_windows: usize) -> Self {
    trace!(
      "Creating new WindowsManager with {display:?} {space:?} {expected_current_num_master_windows}",
      expected_current_num_master_windows = expected_current_num_master_windows.blue()
    );
    Self { display, expected_current_num_master_windows, space, windows: vec![] }
  }

  pub fn windows(&self) -> &Vec<Window> { &self.windows }

  pub fn initialize(&mut self) -> Result<()> {
    debug!("Initializing window manager");
    self.windows = self.get_windows_data()?;

    Ok(())
  }

  pub(crate) fn get_windows_data(&self) -> Result<Vec<Window>> {
    debug!("Reading windows data from yabai");
    let windows = YabaiMessage::query().windows()?;
    debug!("Found {len} windows", len = windows.len().blue());
    let windows: Vec<Window> = windows
      .into_iter()
      .filter(|window| {
        debug!(
          "{window}: {} - {} - {} - {}",
          print_bool!(window.is_floating, "floating", "not floating"),
          print_bool!(window.is_minimized, "minimized", "not minimized"),
          print_bool!(window.is_hidden, "hidden", "not hidden"),
          print_bool!(window.is_visible, "visible", "invisible")
        );
        if window.is_floating || self.display.index != window.display || self.space.index != window.space {
          return false;
        }

        !(window.is_minimized || window.is_hidden || !window.is_visible)
      })
      .collect();
    debug!("{len} windows are handled by ymsp", len = windows.len().blue());

    Ok(windows)
  }

  pub fn validate_state(&mut self, state: &mut State) -> Result<()> {
    debug!("Validating state: {state:?}");
    let space_state = state.get_space_mut(&self.space)?;

    let len = self.windows.len();
    debug!(
      "Current number of windows: {len} < {expected}",
      len = len.blue(),
      expected = self.expected_current_num_master_windows.blue()
    );
    if len < self.expected_current_num_master_windows {
      self.expected_current_num_master_windows = len;
      debug!("Expected number of master windows: {len}", len = len);
      *space_state = len;
    }
    debug!("State validated: {state:?}");

    write_state(state)?;

    Ok(())
  }

  pub(crate) fn get_focused_window(&self) -> Option<&Window> { self.windows.iter().find(|window| window.has_focus) }

  pub(crate) fn is_windows_touching_left_edge(&self, window: &Window) -> Result<bool> {
    let left_padding = YabaiMessage::config().left_padding()?;
    trace!(
      "Checking if {window} is touching the left edge {x} {dx}",
      x = window.frame.x.bright_blue(),
      dx = self.display.frame.x.bright_blue()
    );
    Ok(window.frame.x == self.display.frame.x + left_padding)
  }

  pub(crate) fn get_updated_window_data(&self, window: &Window) -> Option<&Window> {
    trace!("Updating window data for {window}");
    self.windows.iter().find(move |win| win.id == window.id)
  }

  pub(crate) fn get_window_data(&self, process_id: usize, window_id: usize) -> color_eyre::Result<&Window> {
    let window = self.windows.iter().find(|window| window.pid == process_id && window.id == window_id);

    window.ok_or(eyre!("Window with id {window_id} and process id {process_id} not found."))
  }
}

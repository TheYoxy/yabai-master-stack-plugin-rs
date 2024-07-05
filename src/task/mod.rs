use std::{cmp::PartialEq, fmt::Formatter};

use clap::{Args, Subcommand};
use clap_complete::Shell;
use color_eyre::owo_colors::OwoColorize;
use log::info;

use crate::{
  task::{
    create_initialized_windows_manager::create_initialized_windows_manager,
    handlers::{
      completion::generate_completion,
      events::{on_yabai_start, window_created, window_moved},
      focus::{focus_down_window, focus_master_window, focus_next_display, focus_previous_display, focus_up_window},
      move_window::{
        close_focused_window, move_window_to_master, move_window_to_next_display, move_window_to_previous_display,
      },
      window_count::{decrease_master_window_count, increase_master_window_count},
    },
    lock::run_locked,
    ymsp_task::YmspTask,
  },
  yabai::config::initialize_config,
};

mod create_initialized_windows_manager;
mod handlers;
pub mod lock;
pub mod ymsp_task;

#[derive(Args, Debug, Eq, PartialEq)]
pub struct CompletionArgs {
  /// The shell to generate the completion script for
  pub shell: Shell,
}

#[derive(Subcommand, Debug, Eq, PartialEq)]
pub enum Task {
  /// Generate shell completion scripts
  #[clap(value_enum)]
  Completion(CompletionArgs),
  /// Base handler for when yabai starts
  OnYabaiStart,
  /// Event handler for when a window is created
  WindowCreated,
  /// Event handler for when a window is moved
  WindowMoved,
  /// Focus the down window in the current space
  FocusDownWindow,
  /// Focus the up window in the current space
  FocusUpWindow,
  /// Increase the number of windows in the master pane
  IncreaseMasterWindowCount,
  /// Decrease the number of windows in the master pane
  DecreaseMasterWindowCount,
  /// Close the focused window
  CloseFocusedWindow,
  /// Focus the next display
  FocusNextDisplay,
  /// Focus the previous display
  FocusPreviousDisplay,
  /// Move the focused window to the next display
  MoveWindowToNextDisplay,
  /// Move the focused window to the previous display
  MoveWindowToPreviousDisplay,
  /// Move the focused window to the master pane
  MoveWindowToMaster,
  /// Focus the master window
  FocusMasterWindow,
}

impl std::fmt::Display for Task {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { write!(f, "{:?}", self) }
}

impl YmspTask for Task {
  fn run(&self) -> color_eyre::Result<()> {
    initialize_config()?;
    let mut state = create_initialized_windows_manager()?;
    info!("Running task {}", self.yellow());

    match self {
      Task::OnYabaiStart => run_locked(|| on_yabai_start(&mut state)),
      Task::WindowCreated => run_locked(|| window_created(&mut state)),
      Task::WindowMoved => run_locked(|| window_moved(&mut state)),
      Task::IncreaseMasterWindowCount => run_locked(|| increase_master_window_count(&mut state)),
      Task::DecreaseMasterWindowCount => run_locked(|| decrease_master_window_count(&mut state)),
      Task::FocusMasterWindow => focus_master_window(),
      Task::FocusUpWindow => run_locked(|| focus_up_window(&mut state)),
      Task::FocusDownWindow => run_locked(|| focus_down_window(&mut state)),
      Task::FocusNextDisplay => focus_next_display(),
      Task::FocusPreviousDisplay => focus_previous_display(),
      Task::MoveWindowToMaster => move_window_to_master(),
      Task::MoveWindowToNextDisplay => move_window_to_next_display(),
      Task::MoveWindowToPreviousDisplay => move_window_to_previous_display(),
      Task::CloseFocusedWindow => close_focused_window(),
      Task::Completion(completion) => generate_completion(completion),
    }
  }
}

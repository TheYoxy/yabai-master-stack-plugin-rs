use std::{cmp::PartialEq, fmt::Formatter};

use clap::{Args, Subcommand};
use clap_complete::Shell;
use color_eyre::owo_colors::OwoColorize;
use log::info;

use crate::{task::{
  create_initialized_windows_manager::create_initialized_windows_manager,
  handlers::{
    events::{on_yabai_start, window_created, window_moved},
    focus::{focus_down_window, focus_master_window, focus_next_display, focus_previous_display, focus_up_window},
    move_window::{
      close_focused_window, move_window_to_master, move_window_to_next_display, move_window_to_previous_display,
    },
    window_count::{decrease_master_window_count, increase_master_window_count},
  },
  lock::run_locked,
  ymsp_task::YmspTask,
}, yabai::config::initialize_config};

mod create_initialized_windows_manager;
pub(crate) mod handlers;
pub(crate) mod lock;
pub(crate) mod ymsp_task;

#[derive(Args, Debug, Eq, PartialEq)]
pub struct CompletionArgs {
  /// The shell to generate the completion script for
  pub shell: Shell,
}

#[derive(Subcommand, Debug, Eq, PartialEq)]
pub enum Task {
  /// Generate shell completion scripts
  #[clap(value_enum)]
  Completions(CompletionArgs),
  /// Base handler for when yabai starts
  OnYabaiStart,
  /// Event handler for when a window is created
  WindowCreated,
  /// Event handler for when a window is moved
  WindowMoved,
  /// Focuses on the window above the currently focused window
  FocusDownWindow,
  /// Focuses on the window below the currently focused window
  FocusUpWindow,
  /// Increases the number of master windows.
  IncreaseMasterWindowCount,
  /// Decreases the number of master windows.
  DecreaseMasterWindowCount,
  /// Quits the currently focused window by Yabai.
  CloseFocusedWindow,
  /// Focus the next display
  FocusNextDisplay,
  /// Focus the previous display
  FocusPreviousDisplay,
  /// Move the focused window to the next display
  MoveToNextDisplay,
  /// Move the focused window to the previous display
  MoveToPreviousDisplay,
  /// Move the focused window to the master pane
  MoveToMaster,
  /// Focus the master window
  FocusMasterWindow,
}

impl std::fmt::Display for Task {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { write!(f, "{:?}", self) }
}

impl YmspTask for Task {
  fn run(&self) -> color_eyre::Result<()> {
    info!("Running task {}", self.yellow());

    initialize_config()?;
    match self {
      Task::OnYabaiStart => {
        run_locked(|| {
          let mut state = create_initialized_windows_manager()?;
          on_yabai_start(&mut state)
        })
      },
      Task::WindowCreated => {
        run_locked(|| {
          let mut state = create_initialized_windows_manager()?;
          window_created(&mut state)
        })
      },
      Task::WindowMoved => {
        run_locked(|| {
          let mut state = create_initialized_windows_manager()?;
          window_moved(&mut state)
        })
      },
      Task::IncreaseMasterWindowCount => {
        run_locked(|| {
          let mut state = create_initialized_windows_manager()?;
          increase_master_window_count(&mut state)
        })
      },
      Task::DecreaseMasterWindowCount => {
        run_locked(|| {
          let mut state = create_initialized_windows_manager()?;
          decrease_master_window_count(&mut state)
        })
      },
      Task::FocusMasterWindow => focus_master_window(),
      Task::FocusUpWindow => {
        run_locked(|| {
          let mut state = create_initialized_windows_manager()?;
          focus_up_window(&mut state)
        })
      },
      Task::FocusDownWindow => {
        run_locked(|| {
          let mut state = create_initialized_windows_manager()?;
          focus_down_window(&mut state)
        })
      },
      Task::FocusNextDisplay => focus_next_display(),
      Task::FocusPreviousDisplay => focus_previous_display(),
      Task::MoveToMaster => move_window_to_master(),
      Task::MoveToNextDisplay => move_window_to_next_display(),
      Task::MoveToPreviousDisplay => move_window_to_previous_display(),
      Task::CloseFocusedWindow => close_focused_window(),
      action => unreachable!("{:?} must not be called", action.red().bold()),
    }
  }
}

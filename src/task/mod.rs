use clap::Subcommand;
use log::info;

use crate::task::create_initialized_windows_manager::create_initialized_windows_manager;
use crate::task::lock_file::lock_file;
use crate::task::on_yabai_start::on_yabai_start;
use crate::task::window_created::window_created;
use crate::task::window_moved::window_moved;
use crate::task::ymsp_task::YmspTask;
use crate::yabai::config::initialize_config;

mod create_initialized_windows_manager;
mod lock_file;
mod on_yabai_start;
mod window_created;
mod window_moved;
pub mod ymsp_task;

#[derive(Subcommand, Debug)]
pub enum Task {
    CloseFocusedWindow,
    DecreaseMasterWindowCount,
    FocusDownWindow,
    FocusUpWindow,
    IncreaseMasterWindowCount,
    OnYabaiStart,
    WindowCreated,
    WindowMoved,
    FocusNextDisplay,
    FocusPreviousDisplay,
    MoveWindowToNextDisplay,
    MoveWindowToPreviousDisplay,
    MoveWindowToMaster,
    FocusMasterWindow,
}

impl YmspTask for Task {
    fn run(&self) -> color_eyre::Result<()> {
        initialize_config()?;
        let mut state = create_initialized_windows_manager()?;
        info!("Running task {self:?}");
        lock_file()?;
        match self {
            Task::OnYabaiStart => on_yabai_start(&mut state),
            Task::CloseFocusedWindow => todo!(),
            Task::DecreaseMasterWindowCount => todo!(),
            Task::FocusDownWindow => todo!(),
            Task::FocusUpWindow => todo!(),
            Task::WindowCreated => window_created(&mut state),
            Task::WindowMoved => window_moved(&mut state),
            Task::FocusNextDisplay => todo!(),
            Task::FocusPreviousDisplay => todo!(),
            Task::MoveWindowToNextDisplay => todo!(),
            Task::MoveWindowToPreviousDisplay => todo!(),
            Task::MoveWindowToMaster => todo!(),
            Task::FocusMasterWindow => todo!(),
            Task::IncreaseMasterWindowCount => todo!(),
        }
    }
}

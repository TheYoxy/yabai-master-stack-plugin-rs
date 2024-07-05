use std::fs;

use color_eyre::eyre::bail;
use color_eyre::owo_colors::OwoColorize;
use log::{debug, trace};

use crate::yabai::command::{get_yabai_command, handle_output_result};
use crate::yabai::config::{get_lockfile, MasterPosition, YabaiDirection};
use crate::yabai::window::Window;
use crate::yabai::window_manager::ctor::WindowsManager;

#[derive(Debug)]
pub enum YabaiCommand<'a> {
    WarpWindow(&'a Window, &'a Window),
    ToggleWindowSplit(&'a Window),
    WarpDirection(&'a Window, &'a MasterPosition),
}

#[derive(Debug)]
pub enum YabaiConfig {
    LeftPadding,
}

impl YabaiConfig {
    pub fn to_yabai_command(&self) -> &str {
        match self {
            YabaiConfig::LeftPadding => "left_padding",
        }
    }
}

impl WindowsManager {
    pub(super) fn run_yabai_command(&self, command: YabaiCommand) -> color_eyre::Result<()> {
        let lockfile = get_lockfile()?;
        let exists = lockfile.try_exists()?;
        if !exists {
            bail!("Lockfile is not longer owned by this process");
        }
        let pid = fs::read_to_string(lockfile)?;
        let pid: u32 = pid.parse()?;
        let current_pid = std::process::id();
        trace!(
            "Checking lockfile pid: {} == {}",
            pid.blue(),
            current_pid.blue()
        );
        if pid != current_pid {
            bail!("Lockfile is not longer owned by this process");
        }

        trace!("Running yabai command: {:?}", command);
        match command {
            YabaiCommand::WarpWindow(window, master_window) => {
                warp_window(window, master_window)?;
            }
            YabaiCommand::ToggleWindowSplit(window) => {
                toggle_window_split(window)?;
            }
            YabaiCommand::WarpDirection(window, position) => {
                warp_direction(window, position)?;
            }
        }

        Ok(())
    }
}

pub(super) fn get_yabai_config<T>(config: YabaiConfig) -> color_eyre::Result<T>
where
    T: serde::de::DeserializeOwned,
{
    debug!("Getting yabai config: {:?}", config);
    let output = get_yabai_command()?
        .args(["-m", "config", config.to_yabai_command()])
        .output()?;
    handle_output_result(&output)?;

    let output = String::from_utf8(output.stdout)?;
    let output = serde_json::from_str(&output)?;
    Ok(output)
}

fn warp_direction(window: &Window, direction: &MasterPosition) -> color_eyre::Result<()> {
    debug!(
        "warping window {window} to {direction} -> {yabai_direction}",
        yabai_direction = direction.to_yabai_direction()
    );
    trace!(
        "Running command: {}",
        format!(
            "yabai -m window {window} --warp {direction}",
            window = window.id,
            direction = direction.to_yabai_direction()
        )
        .yellow()
    );

    let output = get_yabai_command()?
        .args([
            "-m",
            "window",
            window.id.to_string().as_str(),
            "--warp",
            direction.to_yabai_direction(),
        ])
        .output()?;

    handle_output_result(&output)?;

    Ok(())
}

fn warp_window(window: &Window, master_window: &Window) -> color_eyre::Result<()> {
    debug!("warping window {window} to master window {master_window}");
    let output = get_yabai_command()?
        .args([
            "-m",
            "window",
            window.id.to_string().as_str(),
            "--warp",
            master_window.id.to_string().as_str(),
        ])
        .output()?;
    handle_output_result(&output)?;

    Ok(())
}

fn toggle_window_split(window: &Window) -> color_eyre::Result<()> {
    use log::debug;
    debug!("splitting window {window}");
    let output = get_yabai_command()?
        .args([
            "-m",
            "window",
            window.id.to_string().as_str(),
            "--toggle",
            "split",
        ])
        .output()?;

    handle_output_result(&output)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::yabai::config::initialize_config;

    use super::*;

    #[test]
    fn test_get_yabai_config() {
        initialize_config().unwrap();
        let config = get_yabai_config::<f64>(YabaiConfig::LeftPadding).unwrap();
        assert_eq!(config, 10.0);
    }
}

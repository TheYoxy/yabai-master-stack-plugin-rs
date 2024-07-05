use std::process::{Command, Output};

use color_eyre::owo_colors::{AnsiColors, OwoColorize};
use log::debug;

use super::config::get_config;

pub fn get_yabai_command() -> color_eyre::Result<Command> {
    let config = get_config()?;
    Ok(Command::new(&config.yabai_path))
}

pub fn handle_output_result(output: &Output) -> color_eyre::Result<()> {
    let status = if output.status.success() {
        output.status.color(AnsiColors::Green)
    } else {
        output.status.color(AnsiColors::Red)
    };
    debug!("output: {status}");
    if !output.status.success() {
        debug!(
            "stdout: {stdout}",
            stdout = String::from_utf8_lossy(&output.stdout)
        );
        debug!(
            "stderr: {stderr}",
            stderr = String::from_utf8_lossy(&output.stderr)
        );
        // #[cfg(debug_assertions)]
        // bail!(
        //     "yabai command failed: {stderr}",
        //     stderr = String::from_utf8_lossy(&output.stderr)
        // );
        // #[cfg(not(debug_assertions))]
        // bail!("yabai command failed");
    }

    Ok(())
}

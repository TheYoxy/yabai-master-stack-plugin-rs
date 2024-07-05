use crate::yabai::config::get_lockfile;
use color_eyre::eyre::bail;
use std::fs;

pub(super) fn lock_file() -> color_eyre::Result<()> {
    let lockfile = get_lockfile()?;
    let exists = lockfile.try_exists()?;
    if exists {
        match fs::remove_file(&lockfile) {
            Ok(_) => {}
            Err(e) => {
                bail!("Could not remove lockfile: {}", e);
            }
        }
    }
    fs::write(lockfile, std::process::id().to_string())?;

    Ok(())
}

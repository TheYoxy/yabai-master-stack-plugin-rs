use fslock::LockFile;

use crate::yabai::config::get_lockfile;

pub fn is_locked() -> color_eyre::Result<bool> {
  let lockfile = get_lockfile()?;
  let lockfile = LockFile::open(&lockfile)?;
  Ok(lockfile.owns_lock())
}

pub(super) fn run_locked<T>(f: T) -> color_eyre::Result<()>
where
  T: FnOnce() -> color_eyre::Result<()>,
{
  let lockfile = get_lockfile()?;
  let mut lockfile = LockFile::open(&lockfile)?;

  lockfile.lock_with_pid()?;
  f()?;
  lockfile.unlock()?;

  Ok(())
}

use std::sync::OnceLock;
use color_eyre::eyre::eyre;

static DRY_MODE: OnceLock<bool> = OnceLock::new();
pub(crate) fn is_dry_mode() -> bool { DRY_MODE.get().is_some_and(|f| *f) }
pub(crate) fn set_dry_mode(value: bool) -> color_eyre::Result<()> {
    DRY_MODE.set(value).map_err(|_| eyre!("Failed to set dry mode"))
}

// Unix only: this target makes and reads symlinks and file modes, and the install sets file modes on unix only.
#![cfg(unix)]

mod end_modes;
mod harnesses;
mod placement;
mod record;
mod support;
mod temp_mode;
mod uninstall;

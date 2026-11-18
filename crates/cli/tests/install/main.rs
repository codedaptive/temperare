// Unix only: this target makes and reads symlinks and file modes, which the install makes on unix only.
#![cfg(unix)]

mod harnesses;
mod placement;
mod record;
mod support;
mod uninstall;

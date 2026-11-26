//! `EnvGuard`: sets one process variable for a test and puts it back when dropped. Process variables are shared by
//! every test thread, so a caller whose file runs several tests holds a lock of its own (`LOCK` in hadolint.rs) while a
//! guard lives; a file with one test needs none.

use std::ffi::{OsStr, OsString};

/// The variable's name and the value it had before `set`, `None` when it was unset.
pub struct EnvGuard(&'static str, Option<OsString>);

impl EnvGuard {
    /// Sets `name` to `value` and remembers the value it replaces.
    pub fn set(name: &'static str, value: impl AsRef<OsStr>) -> Self {
        let previous = std::env::var_os(name);
        std::env::set_var(name, value);
        Self(name, previous)
    }
}

impl Drop for EnvGuard {
    /// Restores the remembered value, or removes the variable when it was unset before `set`.
    fn drop(&mut self) {
        match &self.1 {
            Some(value) => std::env::set_var(self.0, value),
            None => std::env::remove_var(self.0),
        }
    }
}

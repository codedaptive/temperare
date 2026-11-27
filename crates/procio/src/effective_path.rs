//! `effective_path`: the PATH a `Cmd`'s child would see. Reads no environment; the caller passes its own PATH.

use std::ffi::{OsStr, OsString};

use crate::Cmd;

/// The child's PATH: the last `cmd.env` entry named PATH, since `env` is applied after `env_remove`; else `None`
/// when `cmd.env_remove` names PATH; else `process_path`, the caller's own.
pub(crate) fn effective_path(cmd: &Cmd, process_path: Option<&OsStr>) -> Option<OsString> {
    if let Some((_, value)) = cmd.env.iter().rev().find(|(name, _)| is_path(name)) {
        return Some(value.clone());
    }
    if cmd.env_remove.iter().any(|name| is_path(name)) {
        return None;
    }
    process_path.map(OsStr::to_os_string)
}

/// Unix variable names are case-sensitive.
#[cfg(not(windows))]
fn is_path(name: &OsStr) -> bool {
    name == "PATH"
}

/// Windows variable names are case-insensitive, so `Path` is PATH too.
#[cfg(windows)]
fn is_path(name: &OsStr) -> bool {
    name.eq_ignore_ascii_case("PATH")
}

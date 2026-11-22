//! `run_golangci_lint`: golangci-lint's flow for one rule, the standard flow with Go kept local.

use crate::entry::{run_plain, Entry};
use crate::host::Host;
use crate::types::{Hit, Stop};
use procio::{Cmd, Output, ProcError};
use rules::Rule;
use std::ffi::OsString;
use std::path::Path;

/// `run_plain` over a host whose `run` gives every command, before `host.run` runs it, the environment
/// GOTOOLCHAIN=local (go downloads no toolchain), GOPROXY=off (no module), GOSUMDB=off (no checksum lookup) and
/// PWD=<the command's folder> (golangci-lint makes its paths from $PWD, not from the real folder), and removes the
/// inherited GOFLAGS (a caller's `-mod=mod` would let go rewrite the checked go.mod and go.sum; without it go reads
/// vendor/ or runs readonly). `on_path`, `raw_chars` and `base` are `host`'s.
pub(crate) fn run_golangci_lint(entry: &Entry, rule: &Rule, folder: &Path, host: &Host) -> Result<Vec<Hit>, Stop> {
    let run = |cmd: &Cmd| -> Result<Output, ProcError> {
        let mut cmd = cmd.clone();
        cmd.env.extend(
            [("GOTOOLCHAIN", "local"), ("GOPROXY", "off"), ("GOSUMDB", "off")]
                .map(|(name, value)| (OsString::from(name), OsString::from(value))),
        );
        cmd.env.extend(cmd.cwd.clone().map(|cwd| (OsString::from("PWD"), cwd.into_os_string())));
        cmd.env_remove.push(OsString::from("GOFLAGS"));
        (host.run)(&cmd)
    };
    run_plain(entry, rule, folder, &Host { run: &run, on_path: host.on_path, raw_chars: host.raw_chars, base: host.base })
}

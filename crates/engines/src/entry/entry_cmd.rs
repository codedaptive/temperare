//! `entry_cmd`: an entry's check run for one rule, as a command.

use super::{Entry, Pick};
use crate::program_for::program_for;
use procio::Cmd;
use std::ffi::{OsStr, OsString};
use std::path::Path;

/// The program `program_for` gives for `cwd` (ESLint's local one when there is one, else `entry.program`) run in
/// `cwd` with, in order: `entry.args`; the pick (`Pick::Arg(flag)`: one argument `{flag}{pick}`; `Pick::File(option)`:
/// `option`, then `pick`; `Pick::None`: nothing, and `pick` is not used); `params`, each one argument as given;
/// `{entry.config}{config}` as one argument when the entry takes a config and `config` is given; then `entry.after`.
/// `config` is passed as the rule wrote it; it is never read here.
pub fn entry_cmd(entry: &Entry, pick: &OsStr, params: &[OsString], config: Option<&str>, cwd: &Path) -> Cmd {
    let mut args: Vec<OsString> = entry.args.iter().map(OsString::from).collect();
    match entry.pick {
        Pick::Arg(flag) => args.push(joined(flag, pick)),
        Pick::File(option) => args.extend([OsString::from(option), pick.to_os_string()]),
        Pick::None => {}
    }
    args.extend_from_slice(params);
    if let (Some(option), Some(config)) = (entry.config, config) {
        args.push(joined(option, OsStr::new(config)));
    }
    args.extend(entry.after.iter().map(OsString::from));
    Cmd { args, cwd: Some(cwd.to_path_buf()), ..Cmd::new(program_for(entry, Some(cwd))) }
}

/// `flag` and `value` as one argument.
fn joined(flag: &str, value: &OsStr) -> OsString {
    let mut arg = OsString::from(flag);
    arg.push(value);
    arg
}

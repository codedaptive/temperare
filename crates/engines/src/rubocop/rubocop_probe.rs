//! `rubocop_probe`: one rubocop run that is not the check run, read as text.

use super::with_rubocop_env;
use crate::host::Host;
use crate::ran::ran;
use crate::types::Stop;
use procio::Cmd;
use std::ffi::OsString;
use std::path::Path;

/// stdout, as lossy UTF-8, of `rubocop <args>` run in `folder` with `with_rubocop_env`'s environment, through
/// `host.run`. It goes through `ran`, not `check_run`, so it is not counted as the rule's check run. A rubocop not
/// found, or that could not be run, stops as `ran` says; an exit other than 0 is `Failed` with that exit code and
/// rubocop's stderr, since tm cannot tell what the repository's config does from a run that gave no answer.
pub(crate) fn rubocop_probe(args: &[&str], folder: &Path, host: &Host) -> Result<String, Stop> {
    let cmd = Cmd { args: args.iter().map(OsString::from).collect(), cwd: Some(folder.to_path_buf()), ..Cmd::new("rubocop") };
    let out = ran(&with_rubocop_env(cmd), host)?;
    if out.code != Some(0) {
        return Err(Stop::Failed { code: out.code, stderr: String::from_utf8_lossy(&out.stderr).into_owned() });
    }
    Ok(String::from_utf8_lossy(&out.stdout).into_owned())
}

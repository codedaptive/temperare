//! `run_markdownlint_with`: markdownlint-cli2's flow for one rule, with the listing and the filesystem given.

use super::{config_stop, tm_options};
use crate::check_run::check_run;
use crate::entry::{entry_cmd, Entry, TempDir};
use crate::host::Host;
use crate::no_script::no_script;
use crate::parse_failed::parse_failed;
use crate::types::{Hit, Stop};
use rules::{RelPath, Rule};
use std::ffi::{OsStr, OsString};
use std::io;
use std::path::{Path, PathBuf};

/// Why a markdownlint rule naming a config is refused: the entry passes no config of the rule's.
const CONFIG_WHY: &str = "a markdownlint rule takes no config: markdownlint reads the repository's own \
    .markdownlint.json or .yaml in the rule file's folder and below";

/// The hits of `rule` under `folder` (canonical), `list` giving the Markdown files git lists under a folder, relative
/// to it, and `is_file` and `read` the filesystem (absolute paths). The central `refused` has already settled the
/// rule's refusals (a check that is not a rule name) and params (markdownlint takes none). Each step can stop it:
/// 1. a rule `config` is `Failed`;
/// 2. the files `list` gives, or its `Stop`; when `no_script` finds none the rule applies to, nothing runs and the
///    rule is its `NoFiles`. `engine_hits_with`'s step 1 (`engine_listed`) settles that before availability; this guard
///    decides only when that listing failed and this one succeeds;
/// 3. `config_stop` on the files the rule applies to, sorted: an options file, a check not in temperare's list, a
///    config tm cannot read or that turns the rule off;
/// 4. tm's options module (`tm_options`) written in a new temporary folder; an error writing it is `Failed`;
/// 5. the check run, through `check_run` so it is counted: `entry_cmd` (`markdownlint-cli2 --no-globs`), then
///    `--config <module>` and each file as `:<file>` (a literal path, never a glob), in `folder`. The module and its
///    folder are removed after it, whatever its outcome, so nothing is left behind and nothing is written in `folder`;
/// 6. its output read by `entry.read`; output it cannot read is `parse_failed`'s `Failed`. Only files `list` gave
///    are passed, so every hit is in a file git lists.
pub(crate) fn run_markdownlint_with(
    list: &dyn Fn(&Path) -> Result<Vec<PathBuf>, Stop>,
    is_file: &dyn Fn(&Path) -> bool,
    read: &dyn Fn(&Path) -> io::Result<String>,
    entry: &Entry,
    rule: &Rule,
    folder: &Path,
    host: &Host,
) -> Result<Vec<Hit>, Stop> {
    if rule.config.is_some() {
        return Err(Stop::Failed { code: None, stderr: CONFIG_WHY.to_string() });
    }
    let listed = list(folder)?;
    if let Some(stop) = no_script(rule, &listed) {
        return Err(stop);
    }
    let mut files: Vec<PathBuf> =
        listed.iter().filter(|file| rule.applies(&RelPath::new(&file.to_string_lossy()))).cloned().collect();
    files.sort();
    if let Some(stop) = config_stop(&rule.check, folder, &files, is_file, read) {
        return Err(stop);
    }
    let (name, text) = tm_options();
    let temp = TempDir::write("markdownlint", name, text).map_err(|err| Stop::Failed {
        code: None,
        stderr: format!("cannot write tm's markdownlint-cli2 options module: {err}"),
    })?;
    let mut cmd = entry_cmd(entry, OsStr::new(&rule.check), &[], None, folder);
    cmd.args.extend([OsString::from("--config"), temp.path.join(name).into_os_string()]);
    cmd.args.extend(files.iter().map(|file| {
        let mut arg = OsString::from(":");
        arg.push(file);
        arg
    }));
    let ran = check_run(&cmd, host);
    drop(temp);
    let out = ran?;
    (entry.read)(&out, &rule.check, folder, folder).map_err(|err| parse_failed(&out, err))
}

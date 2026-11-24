//! `run_mypy`: mypy's flow for one rule. It runs git and the engine and reads the config from disk, so its test is
//! the zone integration test tests/mypy.rs and it has no unit-test sibling.

use super::{config_suppression, mypy_cmd, mypy_config, python_files};
use crate::check_run::check_run;
use crate::entry::Entry;
use crate::host::Host;
use crate::no_script::no_script;
use crate::parse_failed::parse_failed;
use crate::repo_top::repo_top;
use crate::types::{Hit, Stop};
use rules::{RelPath, Rule};
use std::path::{Path, PathBuf};

/// The hits of `rule` under `folder` (canonical); a `RunFn`, the flow of `MYPY`. The central `refused` has already
/// settled the rule's refusals and params (mypy takes none). Each step can stop the rule:
/// 1. the Python files `python_files` lists under `folder`, or its `Stop`; when `no_script` finds none the rule applies
///    to, nothing runs and the rule is its `NoFiles`. `engine_hits_with`'s step 1 (`engine_listed`) settles that before
///    availability; this guard decides only when that listing failed and this one succeeds;
/// 2. the config tm pins: the rule's `config` joined to `folder`, else the one `mypy_config` finds from `folder` up to
///    the repository's top (`repo_top`; `folder` itself when that fails, so the search stays in `folder`), else none;
/// 3. a pinned config that cannot be read, or that `config_suppression` says silences the check, is `Failed` before
///    anything runs;
/// 4. the check run of `mypy_cmd` on the files the rule applies to, in `folder`, through `check_run` so it is counted;
/// 5. mypy saying it does not know the check (`entry.unknown`) is `UnknownCheck`;
/// 6. otherwise the output read by `entry.read`; output it cannot read is `parse_failed`'s `Failed`.
pub(crate) fn run_mypy(entry: &Entry, rule: &Rule, folder: &Path, host: &Host) -> Result<Vec<Hit>, Stop> {
    let files = python_files(folder)?;
    if let Some(stop) = no_script(rule, &files) {
        return Err(stop);
    }
    let kept: Vec<PathBuf> =
        files.into_iter().filter(|file| rule.applies(&RelPath::new(&file.to_string_lossy()))).collect();
    let config = match &rule.config {
        Some(config) => Some(folder.join(config)),
        None => mypy_config(
            folder,
            &repo_top(folder).unwrap_or_else(|_| folder.to_path_buf()),
            &|path| std::fs::read_to_string(path).ok(),
        ),
    };
    if let Some(config) = &config {
        silenced(config, folder, &rule.check)?;
    }
    let out = check_run(&mypy_cmd(entry, &rule.check, config.as_deref(), &kept, folder), host)?;
    if let Some(said) = (entry.unknown)(&out, &rule.check) {
        return Err(Stop::UnknownCheck { said });
    }
    (entry.read)(&out, &rule.check, folder, folder).map_err(|err| parse_failed(&out, err))
}

/// `Failed` when `config` cannot be read or silences `check`'s findings outside the rule file. The why names the
/// config relative to `folder` when it is under it, else by its full path.
fn silenced(config: &Path, folder: &Path, check: &str) -> Result<(), Stop> {
    let name = config.strip_prefix(folder).unwrap_or(config).display().to_string();
    let failed = |stderr: String| Stop::Failed { code: None, stderr };
    let text =
        std::fs::read_to_string(config).map_err(|err| failed(format!("mypy config {name} cannot be read: {err}")))?;
    match config_suppression(&name, &text, check) {
        Ok(None) => Ok(()),
        Ok(Some(why)) | Err(why) => Err(failed(why)),
    }
}

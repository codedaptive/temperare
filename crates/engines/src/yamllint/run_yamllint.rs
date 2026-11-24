//! `run_yamllint`: yamllint's flow for one rule. It runs git and the engine, so its test is the zone integration test
//! tests/yamllint.rs and it has no unit-test sibling.

use super::{config_data, is_yaml_source, yamllint_config};
use crate::check_run::check_run;
use crate::entry::{entry_cmd, Entry};
use crate::host::Host;
use crate::no_script::no_script;
use crate::parse_failed::parse_failed;
use crate::repo_top::repo_top;
use crate::source_files::source_files;
use crate::types::{Hit, Stop};
use rules::{RelPath, Rule};
use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};

/// The hits of `rule` under `folder` (canonical); a `RunFn`, the flow of `YAMLLINT`. The central `refused` has already
/// settled the rule's refusals (a check that is not one rule id) and params (yamllint takes none). Each step can stop it:
/// 1. the YAML files `source_files` lists under `folder` with `is_yaml_source`, or its `Stop`; when `no_script` finds
///    none the rule applies to, nothing runs and the rule is its `NoFiles`. `engine_hits_with`'s step 1 (`engine_listed`)
///    settles that before availability; this guard decides only when that listing failed and this one succeeds. The
///    files kept are those the rule applies to, relative to `folder`, where yamllint runs;
/// 2. the base config (`yamllint_config`), looked for from `folder` up to the repository top (`repo_top`); a `folder`
///    in no repository is `Failed` with git's error. A config the rule names that is not a file is `Failed` naming it,
///    since yamllint given a missing base prints only a traceback;
/// 3. the `-d` text (`config_data`), which turns off the config's ignores, so every file passed is linted; a base path
///    that is not UTF-8 is `Failed`;
/// 4. the check run: `entry_cmd`, `-d <text>`, `--` and the files, through `check_run` so it is counted; yamllint
///    saying it has no such rule (`entry.unknown`) is `UnknownCheck`, else the output read by `entry.read`; output it
///    cannot read is `parse_failed`'s `Failed`.
pub(crate) fn run_yamllint(entry: &Entry, rule: &Rule, folder: &Path, host: &Host) -> Result<Vec<Hit>, Stop> {
    let files = source_files(folder, is_yaml_source)?;
    if let Some(stop) = no_script(rule, &files) {
        return Err(stop);
    }
    let files: Vec<PathBuf> =
        files.into_iter().filter(|file| rule.applies(&RelPath::new(&file.to_string_lossy()))).collect();
    let top = repo_top(folder)?;
    let base = yamllint_config(rule.config.as_deref(), folder, &top, &|path| path.is_file());
    if let Some(path) = base.as_deref().filter(|path| rule.config.is_some() && !path.is_file()) {
        return Err(failed(format!("the rule's yamllint config {} is not a file", path.display())));
    }
    let data = config_data(base.as_deref(), &rule.check).map_err(failed)?;
    let mut cmd = entry_cmd(entry, OsStr::new(&rule.check), &[], None, folder);
    cmd.args.extend([OsString::from("-d"), OsString::from(data), OsString::from("--")]);
    cmd.args.extend(files.into_iter().map(PathBuf::into_os_string));
    let out = check_run(&cmd, host)?;
    if let Some(said) = (entry.unknown)(&out, &rule.check) {
        return Err(Stop::UnknownCheck { said });
    }
    (entry.read)(&out, &rule.check, folder, folder).map_err(|err| parse_failed(&out, err))
}

fn failed(stderr: String) -> Stop {
    Stop::Failed { code: None, stderr }
}

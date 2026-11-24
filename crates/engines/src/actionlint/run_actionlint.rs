//! `run_actionlint`: actionlint's flow for one rule. It reads the disk and runs git and the engine, so its test is the
//! zone integration test tests/actionlint.rs and it has no unit-test sibling.

use super::{actionlint_config, config_suppression, integration_args, is_workflow};
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

/// The hits of `rule` under `folder` (canonical); a `RunFn`, the flow of `ACTIONLINT`. The central `refused` has
/// already settled the rule's refusals (a check that is not a kind's form) and params (actionlint takes none). Each
/// step can stop it:
/// 1. the workflows `source_files` lists under `folder` with `is_workflow`, or its `Stop`; when `no_script` finds none
///    the rule applies to, nothing runs and the rule is its `NoFiles`. `engine_hits_with`'s step 1
///    (`engine_listed`) settles that before availability; this guard decides only when that listing failed and
///    this one succeeds;
/// 2. the repository top (`repo_top`); a `folder` in no repository is `Failed` with git's error;
/// 3. the config tm passes (`actionlint_config`: the rule's, else the top's `.github/actionlint.yaml` or `.yml`);
///    when there is one, its text, a file it cannot read being `Failed` naming it, and `config_suppression`: an
///    `ignore` in it, or text tm cannot read, is `Failed` naming the file and line (S-67 (1));
/// 4. the integration the check turns on (`integration_args`): its program not on PATH (`host.on_path`) is
///    `NotOnPath`, since actionlint would skip the kind silently;
/// 5. the check run, through `check_run` so it is counted: `entry_cmd` (no pick, params or config), then
///    `-config-file=` with the config or `/dev/null`, so actionlint discovers nothing tm did not read, the two
///    integration arguments, `--` and the workflows the rule applies to, relative to `folder`, where it runs; with
///    `SHELLCHECK_OPTS` removed from its environment, since it can silence ShellCheck findings;
/// 6. the check not among the kinds the run defined (`entry.unknown`) is `UnknownCheck`;
/// 7. otherwise the output read by `entry.read`; output it cannot read is `parse_failed`'s `Failed`.
pub(crate) fn run_actionlint(entry: &Entry, rule: &Rule, folder: &Path, host: &Host) -> Result<Vec<Hit>, Stop> {
    let files = source_files(folder, is_workflow)?;
    if let Some(stop) = no_script(rule, &files) {
        return Err(stop);
    }
    let top = repo_top(folder)?;
    let config = actionlint_config(rule.config.as_deref(), folder, &top, &|path| path.is_file());
    if let Some(path) = &config {
        let text = std::fs::read_to_string(path).map_err(|err| Stop::Failed {
            code: None,
            stderr: format!("cannot read the actionlint config {}: {err}", path.display()),
        })?;
        if let Some(stop) = config_suppression(path, &text) {
            return Err(stop);
        }
    }
    let (needs, integrations) = integration_args(&rule.check);
    if let Some(program) = needs.filter(|program| !(host.on_path)(program)) {
        return Err(Stop::NotOnPath { program: program.to_string() });
    }
    let mut cmd = entry_cmd(entry, OsStr::new(""), &[], None, folder);
    let mut config_arg = OsString::from("-config-file=");
    config_arg.push(config.as_deref().unwrap_or(Path::new("/dev/null")));
    cmd.args.push(config_arg);
    cmd.args.extend(integrations.map(OsString::from));
    cmd.args.push(OsString::from("--"));
    cmd.args.extend(
        files
            .into_iter()
            .filter(|file| rule.applies(&RelPath::new(&file.to_string_lossy())))
            .map(PathBuf::into_os_string),
    );
    cmd.env_remove.push(OsString::from("SHELLCHECK_OPTS"));
    let out = check_run(&cmd, host)?;
    if let Some(said) = (entry.unknown)(&out, &rule.check) {
        return Err(Stop::UnknownCheck { said });
    }
    (entry.read)(&out, &rule.check, folder, folder).map_err(|err| parse_failed(&out, err))
}

//! `run_vale`: Vale's flow for one rule. It runs git and the engine and writes a temporary config, so its test is the
//! zone integration test tests/vale.rs and it has no unit-test sibling.

use super::{config_stop, is_vale_source, vale_filter, vale_wrapper, ValeConfig};
use crate::check_run::check_run;
use crate::entry::{entry_cmd, Entry, TempDir};
use crate::host::Host;
use crate::no_script::no_script;
use crate::parse_failed::parse_failed;
use crate::ran::ran;
use crate::repo_top::repo_top;
use crate::source_files::source_files;
use crate::types::{Hit, Stop};
use procio::Cmd;
use rules::{RelPath, Rule};
use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};

/// The hits of `rule` under `folder` (canonical); a `RunFn`, the flow of `VALE`. The central `refused` has already
/// settled the rule's refusals (a check that is not `Style.Rule`) and params (Vale takes none). Each step can stop it:
/// 1. the files `source_files` lists under `folder` with `is_vale_source`, or its `Stop`; when `no_script` finds none
///    the rule applies to, nothing runs and the rule is its `NoFiles`. `engine_hits_with`'s step 1 (`engine_listed`)
///    settles that before availability; this guard decides only when that listing failed and this one succeeds;
/// 2. the repository's config, the rule's own when it names one (it stands in for a `.vale.ini`):
///    `vale --no-global [--config=<the rule's config, canonical when it exists>] ls-config` run in `folder` through
///    `ran`, not counted, read by `ValeConfig::read` (no config is `NotAvailable`; a config Vale cannot load, or a
///    rule's config it cannot find, `Failed`).
///    The probe runs without `PWD`, so Vale takes its folder from the system, in `folder`'s canonical form, and the
///    config paths it prints compare with the canonical repository top;
/// 3. the repository top (`repo_top`, or its `Stop`) and `config_stop`: a config outside it, or a check whose style
///    no `BasedOnStyles` names, is `NotAvailable`;
/// 4. the wrapper config's text (`vale_wrapper`, for the check's style); a value it cannot write is `Failed`;
/// 5. the wrapper written as `.vale.ini` in a new temporary folder; an error writing it is `Failed`;
/// 6. the check run, through `check_run` so it is counted: `entry_cmd`, then `vale_filter`'s argument,
///    `--config=<wrapper>`, `--` and the files the rule applies to, relative to `folder`, where it runs. After it,
///    whatever its outcome, the wrapper and its folder are removed, so the run leaves nothing behind;
/// 7. Vale saying no loaded rule is the check (`entry.unknown`) is `UnknownCheck`;
/// 8. otherwise the output read by `entry.read`; output it cannot read is `parse_failed`'s `Failed`.
pub(crate) fn run_vale(entry: &Entry, rule: &Rule, folder: &Path, host: &Host) -> Result<Vec<Hit>, Stop> {
    let files = source_files(folder, is_vale_source)?;
    if let Some(stop) = no_script(rule, &files) {
        return Err(stop);
    }
    let config = ValeConfig::read(&ran(&probe_cmd(entry, rule, folder), host)?)?;
    let top = repo_top(folder)?;
    if let Some(stop) = config_stop(&config, &rule.check, &top) {
        return Err(stop);
    }
    let style = rule.check.split_once('.').map_or(rule.check.as_str(), |(style, _)| style);
    let text = vale_wrapper(&config, style).map_err(|stderr| Stop::Failed { code: None, stderr })?;
    let temp = TempDir::write("vale", ".vale.ini", &text).map_err(|err| Stop::Failed {
        code: None,
        stderr: format!("cannot write the vale wrapper config: {err}"),
    })?;
    let mut cmd = entry_cmd(entry, OsStr::new(""), &[], None, folder);
    let mut wrapper = OsString::from("--config=");
    wrapper.push(temp.path.join(".vale.ini"));
    cmd.args.extend([OsString::from(vale_filter(&rule.check)), wrapper, OsString::from("--")]);
    cmd.args.extend(
        files
            .into_iter()
            .filter(|file| rule.applies(&RelPath::new(&file.to_string_lossy())))
            .map(PathBuf::into_os_string),
    );
    let ran = check_run(&cmd, host);
    drop(temp);
    let out = ran?;
    if let Some(said) = (entry.unknown)(&out, &rule.check) {
        return Err(Stop::UnknownCheck { said });
    }
    (entry.read)(&out, &rule.check, folder, folder).map_err(|err| parse_failed(&out, err))
}

/// `vale --no-global [--config=<rule.config>] ls-config`, run in `folder` with `PWD` removed from its environment.
/// The rule's config is joined to `folder` and canonicalized, because Vale prints a relative `--config` back as given
/// in `ConfigFiles` and `config_stop` compares canonical paths with the repository top (a `..` is resolved, so a
/// config that climbs out of the repository is outside it). One that does not exist is passed as joined; Vale answers
/// `path '<it>' does not exist` on stderr, exit 2, and the rule is `Failed` (fixtures/fixtures.txt, "Probe
/// `--config`").
fn probe_cmd(entry: &Entry, rule: &Rule, folder: &Path) -> Cmd {
    let config = rule.config.iter().map(|config| {
        let path = folder.join(config);
        let mut arg = OsString::from("--config=");
        arg.push(std::fs::canonicalize(&path).unwrap_or(path));
        arg
    });
    Cmd {
        args: std::iter::once(OsString::from("--no-global")).chain(config).chain([OsString::from("ls-config")]).collect(),
        cwd: Some(folder.to_path_buf()),
        env_remove: vec![OsString::from("PWD")],
        ..Cmd::new(entry.program)
    }
}

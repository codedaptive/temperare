//! `run_phpstan`: PHPStan's flow for one rule. It runs git and the engine and writes a temporary config, so its test
//! is the zone integration test tests/phpstan.rs and it has no unit-test sibling.

use super::{is_php_source, no_php_analysed, phpstan_config, repo_config};
use crate::check_run::check_run;
use crate::entry::{entry_cmd, Entry, TempDir};
use crate::host::Host;
use crate::no_script::no_script;
use crate::parse_failed::parse_failed;
use crate::source_files::source_files;
use crate::types::{Hit, Stop};
use rules::{RelPath, Rule};
use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};

/// Why a rule with no PHPStan config to include is not available: it names the six files `repo_config` looks for.
const NO_CONFIG_WHY: &str = "the rule names no config and its folder holds no phpstan.neon, phpstan.neon.dist or \
    phpstan.dist.neon, nor their dot-prefixed forms: cctao runs phpstan with the repository's own config";

/// The hits of `rule` under `folder` (canonical); a `RunFn`, the flow of `PHPSTAN`. The central `refused` has already
/// settled the rule's refusals (a check that is not a level) and params (PHPStan takes none). Each step can stop it:
/// 1. the PHP files `source_files` lists under `folder` with `is_php_source`, or its `Stop`; when `no_script` finds none the rule applies to,
///    nothing runs and the rule is its `NoFiles`. `engine_hits_with`'s step 1 (`engine_listed`) settles that before
///    availability; this guard decides only when that listing failed and this one succeeds;
/// 2. the config to include (`repo_config`: the rule's, else the first file in `folder` of those PHPStan itself
///    discovers, in its order); none is `NotAvailable`;
/// 3. the wrapper config's text (`phpstan_config`); a path it cannot write is `Failed`;
/// 4. the wrapper written as `cctao.neon` in a new temporary folder; an error writing it is `Failed`;
/// 5. the check run, through `check_run` so it is counted: `entry_cmd` with the rule's check as the level, then
///    `--configuration=<wrapper>`, `--` and the files the rule applies to, relative to `folder`, where it runs. After
///    it, whatever its outcome, the `cache` folder PHPStan made as its `tmpDir` is removed (an error is ignored: a
///    cache PHPStan never wrote is absent), then the wrapper and its folder, so the run leaves nothing behind;
/// 6. PHPStan saying it has no such level (`entry.unknown`) is `UnknownCheck`;
/// 7. PHPStan saying no file was left to analyse (`no_php_analysed`) is `no_script`'s `NoFiles`, given no file. The
///    wrapper empties `excludePaths`, so this is the config's `stubFiles` naming every file passed: PHPStan reads a
///    stub for its types and never analyses it;
/// 8. otherwise the output read by `entry.read`; output it cannot read is `parse_failed`'s `Failed`.
pub(crate) fn run_phpstan(entry: &Entry, rule: &Rule, folder: &Path, host: &Host) -> Result<Vec<Hit>, Stop> {
    let files = source_files(folder, is_php_source)?;
    if let Some(stop) = no_script(rule, &files) {
        return Err(stop);
    }
    let config = repo_config(rule.config.as_deref(), folder, &|path| path.is_file())
        .ok_or_else(|| Stop::NotAvailable { why: NO_CONFIG_WHY.to_string() })?;
    let text = phpstan_config(&config).map_err(|stderr| Stop::Failed { code: None, stderr })?;
    let temp = TempDir::write("phpstan", "cctao.neon", &text).map_err(|err| Stop::Failed {
        code: None,
        stderr: format!("cannot write the phpstan wrapper config: {err}"),
    })?;
    let mut cmd = entry_cmd(entry, OsStr::new(&rule.check), &[], None, folder);
    let mut wrapper = OsString::from("--configuration=");
    wrapper.push(temp.path.join("cctao.neon"));
    cmd.args.extend([wrapper, OsString::from("--")]);
    cmd.args.extend(
        files
            .into_iter()
            .filter(|file| rule.applies(&RelPath::new(&file.to_string_lossy())))
            .map(PathBuf::into_os_string),
    );
    let ran = check_run(&cmd, host);
    let _ = std::fs::remove_dir_all(temp.path.join("cache"));
    drop(temp);
    let out = ran?;
    if let Some(said) = (entry.unknown)(&out, &rule.check) {
        return Err(Stop::UnknownCheck { said });
    }
    // `no_script` given no file always gives its `NoFiles`, so this case carries the same why as step 1.
    if let Some(stop) = no_php_analysed(&out).then(|| no_script(rule, &[])).flatten() {
        return Err(stop);
    }
    (entry.read)(&out, &rule.check, folder, folder).map_err(|err| parse_failed(&out, err))
}

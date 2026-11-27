//! `run_hadolint`: hadolint's flow for one rule. It runs git and the engine and reads and writes config files, so its
//! test is the zone integration test tests/hadolint.rs and it has no unit-test sibling.

use super::{hadolint_cmd, hadolint_config, hadolint_vars, ignored_listed, is_dockerfile, not_listed};
use crate::check_run::check_run;
use crate::entry::{Entry, TempFile};
use crate::failed::failed;
use crate::host::Host;
use crate::no_script::no_script;
use crate::parse_failed::parse_failed;
use crate::source_files::source_files;
use crate::types::{Hit, Stop};
use rules::{RelPath, Rule};
use std::path::{Path, PathBuf};

/// The hits of `rule` under `folder` (canonical); a `RunFn`, the flow of `HADOLINT`. The central `refused` has already
/// settled the rule's refusals (`hadolint_refusal`) and params (hadolint takes none). Each step can stop the rule:
/// 1. the Dockerfiles `source_files` lists under `folder` with `is_dockerfile`, or its `Stop`; when `no_script` finds
///    none the rule applies to, nothing runs and the rule is its `NoFiles`. `engine_hits_with`'s step 1
///    (`no_file_listed`) settles that before availability; this guard decides only when that listing failed and
///    this one succeeds;
/// 2. a check not in cctao's list of hadolint codes is `not_listed`'s `NotListed`, so nothing runs for it;
/// 3. the config cctao pins (`hadolint_config`). A pinned config that cannot be read, or whose `ignored` hides findings
///    (`ignored_listed`), is `Failed` before anything runs; with none, a temporary file holding `{}` is pinned in its
///    place, and an error writing it is `Failed`;
/// 4. the check run of `hadolint_cmd` on the files the rule applies to, in `folder`, through `check_run` so it is
///    counted, with every variable in cctao's environment whose name starts with `HADOLINT_` (`hadolint_vars`) removed
///    from the run's; the temporary config lives until the run has ended;
/// 5. the output read by `entry.read`; output it cannot read is `parse_failed`'s `Failed`.
pub(crate) fn run_hadolint(entry: &Entry, rule: &Rule, folder: &Path, host: &Host) -> Result<Vec<Hit>, Stop> {
    let files = source_files(folder, is_dockerfile)?;
    if let Some(stop) = no_script(rule, &files) {
        return Err(stop);
    }
    let kept: Vec<PathBuf> =
        files.into_iter().filter(|file| rule.applies(&RelPath::new(&file.to_string_lossy()))).collect();
    not_listed(include_str!("hadolint_codes.txt"), &rule.check)?;
    // `_empty` keeps the temporary config on disk until the run below has ended; dropping it removes the file.
    let (config, _empty) = match hadolint_config(folder, rule.config.as_deref(), &|path| path.is_file()) {
        Some(config) => {
            unignored(&config, folder)?;
            (config, None)
        }
        None => {
            let temp = TempFile::write("hadolint-config", "yaml", "{}\n").map_err(|err| failed(format!("cannot write the empty hadolint config: {err}")))?;
            (temp.path.clone(), Some(temp))
        }
    };
    let removed = hadolint_vars(std::env::vars_os().map(|(name, _)| name));
    let out = check_run(&hadolint_cmd(entry, &rule.check, &config, &kept, folder, &removed), host)?;
    (entry.read)(&out, &rule.check, folder, folder).map_err(|err| parse_failed(&out, err))
}

/// `Failed` when `config` cannot be read or its `ignored` hides findings outside the rule file. The why names the
/// config relative to `folder` when it is under it, else by its full path.
fn unignored(config: &Path, folder: &Path) -> Result<(), Stop> {
    let name = config.strip_prefix(folder).unwrap_or(config).display().to_string();
    let text =
        std::fs::read_to_string(config).map_err(|err| failed(format!("hadolint config {name} cannot be read: {err}")))?;
    ignored_listed(&name, &text).map_or(Ok(()), |why| Err(failed(why)))
}

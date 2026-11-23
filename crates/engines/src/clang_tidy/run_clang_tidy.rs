//! `run_clang_tidy`: clang-tidy's flow for one rule. It runs git and the engine, so its test is the zone integration
//! test tests/clang_tidy.rs and it has no unit-test sibling.

use super::{c_files, clang_tidy_cmd, no_compile_database};
use crate::check_run::check_run;
use crate::entry::Entry;
use crate::host::Host;
use crate::no_script::no_script;
use crate::parse_failed::parse_failed;
use crate::types::{Hit, Stop};
use rules::{RelPath, Rule};
use std::path::{Path, PathBuf};

/// The hits of `rule` under `folder` (canonical); a `RunFn`, the flow of `CLANG_TIDY`. The central `refused` has
/// already settled the rule's refusals and params (clang-tidy takes none). Each step can stop the rule:
/// 1. the C-family sources `c_files` lists under `folder`, or its `Stop`; when `no_script` finds none the rule
///    applies to, nothing runs and the rule is its `NoFiles`. `engine_hits_with`'s step 1 (`c_listed`) settles that
///    before availability; this guard decides only when that listing failed and this one succeeds;
/// 2. the check run of `clang_tidy_cmd` on the sources the rule applies to, with the rule's config, in `folder`,
///    through `check_run` so it is counted;
/// 3. clang-tidy saying no check is enabled (`entry.unknown`) is `UnknownCheck`: the run enables only the rule's
///    check, so the engine does not know it;
/// 4. a source with no compile database or compile flags (`no_compile_database`) is `NotAvailable`: clang-tidy then
///    checks it without flags and exits 0, so its findings would not be the project's;
/// 5. otherwise the output read by `entry.read`; output it cannot read is `parse_failed`'s `Failed`.
pub(crate) fn run_clang_tidy(entry: &Entry, rule: &Rule, folder: &Path, host: &Host) -> Result<Vec<Hit>, Stop> {
    let files = c_files(folder)?;
    if let Some(stop) = no_script(rule, &files) {
        return Err(stop);
    }
    let kept: Vec<PathBuf> =
        files.into_iter().filter(|file| rule.applies(&RelPath::new(&file.to_string_lossy()))).collect();
    let out = check_run(&clang_tidy_cmd(entry, &rule.check, rule.config.as_deref(), &kept, folder), host)?;
    if let Some(said) = (entry.unknown)(&out, &rule.check) {
        return Err(Stop::UnknownCheck { said });
    }
    if let Some(why) = no_compile_database(&out) {
        return Err(Stop::NotAvailable { why });
    }
    (entry.read)(&out, &rule.check, folder, folder).map_err(|err| parse_failed(&out, err))
}

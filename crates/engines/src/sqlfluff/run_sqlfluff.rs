//! `run_sqlfluff`: sqlfluff's flow for one rule. It runs git and the engine, so its test is the zone integration test
//! tests/sqlfluff.rs and it has no unit-test sibling.

use super::{is_sql_source, no_dialect, sqlfluff_cmd, unreported_files};
use crate::check_run::check_run;
use crate::entry::Entry;
use crate::host::Host;
use crate::no_script::no_script;
use crate::parse_failed::parse_failed;
use crate::repo_top::repo_top;
use crate::source_files::source_files;
use crate::types::{Hit, Stop};
use rules::{RelPath, Rule};
use std::path::{Path, PathBuf};

/// The hits of `rule` under `folder` (canonical); a `RunFn`, the flow of `SQLFLUFF`. The central `refused` has already
/// settled the rule's refusals (a check that is not one rule reference) and params (sqlfluff takes none). Each step
/// can stop the rule:
/// 1. the SQL files `source_files` lists under `folder` with `is_sql_source`, or its `Stop`; when `no_script` finds
///    none the rule applies to, nothing runs and the rule is its `NoFiles`. `engine_hits_with`'s step 1
///    (`engine_listed`) settles that before availability; this guard decides only when that listing failed and this one
///    succeeds;
/// 2. the home sqlfluff is given: the repository's top above `folder` (`repo_top`), so it reads only the repository's
///    configs; `folder` itself when `repo_top` fails, so the config search starts no higher than `folder`;
/// 3. the check run of `sqlfluff_cmd` on the files the rule applies to, with the rule's config, in `folder`, through
///    `check_run` so it is counted;
/// 4. sqlfluff refusing to lint for want of a dialect (`no_dialect`) is `NotAvailable` with that why;
/// 5. sqlfluff saying it does not know the check (`entry.unknown`) is `UnknownCheck`;
/// 6. a file it was handed and left out of its report (`unreported_files`) is `Failed` with that why and its exit code;
/// 7. otherwise the output read by `entry.read`; output it cannot read is `parse_failed`'s `Failed`.
pub(crate) fn run_sqlfluff(entry: &Entry, rule: &Rule, folder: &Path, host: &Host) -> Result<Vec<Hit>, Stop> {
    let files = source_files(folder, is_sql_source)?;
    if let Some(stop) = no_script(rule, &files) {
        return Err(stop);
    }
    let kept: Vec<PathBuf> =
        files.into_iter().filter(|file| rule.applies(&RelPath::new(&file.to_string_lossy()))).collect();
    let home = repo_top(folder).unwrap_or_else(|_| folder.to_path_buf());
    let out = check_run(&sqlfluff_cmd(entry, &rule.check, rule.config.as_deref(), &kept, folder, &home), host)?;
    if let Some(why) = no_dialect(&out) {
        return Err(Stop::NotAvailable { why });
    }
    if let Some(said) = (entry.unknown)(&out, &rule.check) {
        return Err(Stop::UnknownCheck { said });
    }
    if let Some(stderr) = unreported_files(&out, &kept) {
        return Err(Stop::Failed { code: out.code, stderr });
    }
    (entry.read)(&out, &rule.check, folder, folder).map_err(|err| parse_failed(&out, err))
}

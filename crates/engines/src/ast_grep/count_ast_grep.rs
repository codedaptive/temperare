//! `count_ast_grep`: one ast-grep rule in count mode, run over its rule file's folder and counted into hits.

use super::{added_in_folder, parse_count_matches, setup_heavier_hits, CountMode};
use crate::check_run::check_run;
use crate::entry::{entry_cmd, Entry, TempFile};
use crate::host::Host;
use crate::parse_failed::parse_failed;
use crate::types::{Hit, Stop};
use crate::ONLY_WITH_BASE_WHY;
use rules::Rule;
use std::path::Path;

/// The hits of `rule` in count mode under `folder` (canonical); `entry` is `AST_GREP` and `filled` the rule YAML with
/// its params filled. With no base (`host.base` None) the rule is `NotAvailable` with `ONLY_WITH_BASE_WHY` and nothing
/// runs. Else `filled` is written to a temporary file, removed before this returns whatever the outcome, that the
/// entry's command runs in `folder` through `host`, counted (`check_run`); a copy that cannot be written is `Failed`
/// with no exit code, ast-grep absent is `NotOnPath`, and output that is not ast-grep's JSON lines is `Failed` with
/// its exit code. Only then is git asked which files the change adds lines to (`added_in_folder`), so a refused check
/// run never runs git; a git error there is `git_stop`'s `Stop` (`NotOnPath` when git is missing, else `Failed` with
/// git's exit code) and a kept regular untracked file that cannot be read is `Failed` naming it. The hits are
/// `setup_heavier_hits`'s.
pub(crate) fn count_ast_grep(
    entry: &Entry,
    rule: &Rule,
    folder: &Path,
    host: &Host,
    filled: &str,
    mode: &CountMode,
) -> Result<Vec<Hit>, Stop> {
    // No base: the only stop for a rule without the only_with_base mark, since engine_hits_with settles a marked
    // rule with no base before this flow.
    let Some(base) = host.base else {
        return Err(Stop::NotAvailable { why: ONLY_WITH_BASE_WHY.to_string() });
    };
    let check = &rule.check;
    let temp = TempFile::write("ast-grep", "yml", filled).map_err(|err| Stop::Failed {
        code: None,
        stderr: format!("cannot write the filled ast-grep rule {check}: {err}"),
    })?;
    let out = check_run(&entry_cmd(entry, temp.path.as_os_str(), &[], None, folder), host)?;
    drop(temp);
    let matches = parse_count_matches(&out).map_err(|err| parse_failed(&out, err))?;
    let added = added_in_folder(folder, base, &|path| rule.applies(path))?;
    Ok(setup_heavier_hits(&matches, folder, folder, &added, mode.shared_setup.as_ref()))
}

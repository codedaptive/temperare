//! `no_file_rules_listed`: the rules of a rule file settled as no-file before any engine runs, given the folder's
//! listings.

use crate::cargo_audit::{has_lock, lock_listed};
use crate::no_file_match::{no_file_match, NO_FILE_MATCH_WHY};
use crate::refused::refused;
use crate::shellcheck::no_script_listed;
use crate::types::{NoFileRule, Stop};
use rules::{RelPath, RuleFile};
use std::path::{Path, PathBuf};

/// The rules of `file`, in rule-file order, that are settled as no-file, each with its engine and why. Each rule is
/// settled in tm check's order, the first that applies deciding it: a rule `refused` refuses is left out; with a
/// `files` listing, a rule `no_file_match` says matches none of the files, with `NO_FILE_MATCH_WHY`; a rule
/// `no_script_listed` gives `Stop::NoFiles` (a ShellCheck rule with a `folder` none of whose scripts, as `scripts`
/// lists them, it applies to), with that why; a rule `lock_listed` gives `Stop::NoFiles` (a cargo-audit rule with a
/// `folder` holding no Cargo.lock, or whose paths do not cover it), with that why. These are `run_rules_listed`'s
/// step 2 and `engine_hits_with`'s step 1, through the same functions. With no `files` listing no rule is settled by
/// its paths, with no `folder` none by its Cargo.lock, and with no `folder`, or a failed script listing, none by its
/// scripts.
/// `only_with_base` plays no part: tm check settles a rule as no-file before its step 1b looks at the base. Runs no
/// engine and looks nothing up on PATH; the calls out are `scripts` and, for a cargo-audit rule, `has_lock`, which
/// reads the disk.
pub(crate) fn no_file_rules_listed(
    file: &RuleFile,
    folder: Option<&Path>,
    files: Option<&[RelPath]>,
    scripts: &dyn Fn(&Path) -> Result<Vec<PathBuf>, Stop>,
) -> Vec<NoFileRule> {
    file.rules
        .iter()
        .filter(|rule| refused(rule).is_none())
        .filter_map(|rule| {
            let why = if files.is_some_and(|files| no_file_match(rule, files)) {
                NO_FILE_MATCH_WHY.to_string()
            } else if let Some(Stop::NoFiles { why }) = no_script_listed(rule, folder, scripts) {
                why
            } else if let Some(Stop::NoFiles { why }) = lock_listed(rule, folder, &has_lock) {
                why
            } else {
                return None;
            };
            Some(NoFileRule { rule: rule.id.clone(), engine: rule.engine, why })
        })
        .collect()
}

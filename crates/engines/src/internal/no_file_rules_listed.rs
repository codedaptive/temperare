//! `no_file_rules_listed`: the rules of a rule file settled as no-file before any engine runs, given the folder's
//! listings.

use crate::no_file_listed::no_file_listed;
use crate::no_file_match::{no_file_match, NO_FILE_MATCH_WHY};
use crate::refused::refused;
use crate::types::{NoFileRule, Stop};
use rules::{RelPath, RuleFile};
use std::path::{Path, PathBuf};

/// The rules of `file`, in rule-file order, that are settled as no-file, each with its engine and why. Each rule is
/// settled in cctao check's order, the first that applies deciding it: a rule `refused` refuses is left out; with a
/// `files` listing, a rule `no_file_match` says matches none of the files, with `NO_FILE_MATCH_WHY`; then a rule
/// `no_file_listed` gives a why in `folder` (ShellCheck listing through `scripts`; its doc lists each engine's
/// lister and their order). These are `run_rules_listed`'s step 2 and `engine_hits_with`'s step 1, through the same
/// functions. With no `files` listing no rule is settled by its paths, and with no `folder`, or a failed listing, none
/// by its folder's files. `only_with_base` plays no part: cctao check settles a rule as no-file before its step 1b looks
/// at the base. Runs no engine and looks nothing up on PATH; the calls out are `scripts` and `no_file_listed`'s
/// listers, which read the disk or run git.
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
            } else if let Some(why) = no_file_listed(rule, folder, scripts) {
                why
            } else {
                return None;
            };
            Some(NoFileRule { rule: rule.id.clone(), engine: rule.engine, why })
        })
        .collect()
}

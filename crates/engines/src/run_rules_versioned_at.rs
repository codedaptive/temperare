//! `run_rules_versioned_at`: every rule of a rule file run by its engine against a base commit, with the version of
//! each engine named.

use crate::check_host::check_host;
use crate::run_rules_versioned_with::run_rules_versioned_with;
use crate::types::CheckedRules;
use rules::{RelPath, RuleFile};
use std::cell::Cell;
use std::path::Path;

/// One check of a rule file. `runs` is each rule's run, in rule-file order. The rule file's folder is `file.folder`,
/// relative to `repo` unless absolute; every finding's path is relative to it. `only` empty checks the whole folder;
/// otherwise findings are cut to the files and folders it lists, a folder keeping what lies under it (engines still
/// run over the folder). A rule tm refuses (its check or params, ast-grep's `{{name}}` params apart) is `Failed`
/// first, whatever its engine; a ShellCheck rule with no script in its paths, a cargo-audit rule with no Cargo.lock in
/// its folder or paths, or a clang-tidy rule with no C-family source in its paths, is no findings with the `no_files` flag, even if its engine is absent; any other rule with `only_with_base` and no `base`, whose paths match a listed file
/// or where no listing is made, is `NotAvailable` with `rules::ONLY_WITH_BASE_WHY`, whatever its engine, and its check
/// never runs; an absent engine is `NotAvailable` with `availability`'s why; a check the engine does not know is
/// `NotAvailable` with a why naming the engine, its version
/// and the check; an engine that ran but gave no answer is `Failed`; otherwise `Findings`, empty when clean. A run's
/// `raw_chars` is what its check run printed, `None` when no run was counted for it; its `Some` values count the check
/// runs, not the rules. `engines` is each
/// distinct engine the rules name, once, in first-appearance order, with the version its probe read (`None` when the
/// engine is not available, has no probe, the folder cannot be resolved, or the probe gave none). Each available
/// engine is probed once per check, before the rules run; no `raw_chars` counts a probe.
///
/// `base` is the commit the check compares against (cli's `--base`), as cli gives it, `None` without one. It goes into
/// the check's host (`check_host`), and every rule's run is given it there (`Host::base`), a rule of a shared group
/// too. Without one, a rule with `only_with_base` is settled as above (its engine is still probed for `engines`). The
/// git entry and ast-grep's count mode read it, marked or not: without one, such a rule is `NotAvailable` with
/// `rules::ONLY_WITH_BASE_WHY`; with one, it counts the change against it. For every rule that is neither marked
/// `only_with_base` nor read by the git entry or ast-grep's count mode, the outcome is the same for any `base`.
pub fn run_rules_versioned_at(file: &RuleFile, repo: &Path, only: &[RelPath], base: Option<&str>) -> CheckedRules {
    run_rules_versioned_with(file, repo, only, &check_host(base, &Cell::new(None)))
}

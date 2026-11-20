//! `run_rules_versioned_with`: every rule of a rule file run through a host, with the version of each engine named.

use crate::engine_versions::engine_versions;
use crate::host::Host;
use crate::rule_folder::rule_folder;
use crate::run_rules_counted_with::run_rules_counted_with;
use crate::types::CheckedRules;
use rules::{RelPath, RuleFile};
use std::path::Path;

/// `run_rules_versioned_at` with the programs run and looked up through `host`: the engines' versions first
/// (`engine_versions`, in the folder resolved for them, `None` each when it cannot be), then the rules' runs
/// (`run_rules_counted_with`), whose too-old whys name those versions. The probes are not counted in any run's
/// `raw_chars`.
pub(crate) fn run_rules_versioned_with(file: &RuleFile, repo: &Path, only: &[RelPath], host: &Host) -> CheckedRules {
    let engines = engine_versions(file, rule_folder(file, repo).ok().as_deref(), host);
    CheckedRules { runs: run_rules_counted_with(file, repo, only, host, &engines), engines }
}

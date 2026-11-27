//! `RulesArgs`: the flags of `cctao rules`.

use std::path::PathBuf;

/// `cctao rules [--rules <path>] [--json]` and `cctao rules --loosened --base <rev> [--rules <path>] [--json]`: the
/// rule file given with `--rules`, whether to print JSON, and with `--loosened` the commit `--base` names, to list what
/// the rule file loosened since it. `parse_rules_args` gives `base` only together with `loosened`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RulesArgs {
    pub rules: Option<PathBuf>,
    pub json: bool,
    pub base: Option<String>,
    pub loosened: bool,
}

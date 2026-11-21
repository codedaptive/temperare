//! rules: the rule file: read it, check it, scope a rule by path and kit, exceptions; the rule and finding types; what a change loosened. Depends on no workspace crate (serde, toml and globset only, S-1). See the Temperare mission (Layout).

mod loosened;
mod rule_file;
mod scope;
mod types;

pub use loosened::{Loosening, LooseningKind, loosened};
pub use rule_file::{Kit, RuleFile, baseline_rules, baseline_sets};
pub use scope::{Date, Exception, Rule, Scope};
pub use types::{CaseFile, Engine, ExpectedFinding, Finding, Kind, ONLY_WITH_BASE_WHY, Problem, RelPath, RuleCases, RuleOutcome, Severity};

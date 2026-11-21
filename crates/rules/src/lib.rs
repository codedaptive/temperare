//! rules: the rule file: read it, check it, scope a rule by path and kit, exceptions; the rule and finding types. Depends on no workspace crate (serde, toml and globset only, S-1). See the Temperare mission (Layout).

mod rule_file;
mod scope;
mod types;

pub use rule_file::{Kit, RuleFile, baseline_rules, baseline_sets};
pub use scope::{Date, Exception, Rule, Scope};
pub use types::{CaseFile, Engine, ExpectedFinding, Finding, Kind, Problem, RelPath, RuleCases, RuleOutcome, Severity};

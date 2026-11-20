//! rules: the rule file: read it, check it, scope a rule by path and kit, exceptions; the rule and finding types. Depends on no workspace crate (serde, toml and globset only).

mod rule_file;
mod scope;
mod types;

pub use rule_file::{Kit, RuleFile};
pub use scope::{Exception, Rule, Scope};
pub use types::{Engine, Finding, Kind, Problem, RelPath, RuleOutcome, Severity};

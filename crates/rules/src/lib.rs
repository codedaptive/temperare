//! rules: the rule file: read it, check it, scope a rule by path and kit, exceptions; the rule and finding types. Depends on no workspace crate (serde, toml and globset only, S-1). See the Temperare mission (Layout).

mod rule_file;
mod scope;
mod types;

pub use rule_file::RuleFile;
pub use scope::{Exception, Rule, Scope};
pub use types::{Engine, Finding, Problem, RelPath, Severity};

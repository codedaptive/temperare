//! `Rule::new`: a rule built in code.

use super::{Rule, Scope};
use crate::{Engine, Kind, Severity};
use globset::GlobSet;

impl Rule {
    /// A rule `id` that runs `engine` on `check`, with the rule file's defaults for kind (Quality) and severity
    /// (Error), no config, no paths, excludes or kits, an empty fix (the rule file requires one) and no params, a
    /// scope covering every file, no `enforces` or `leaves`, `only_with_base` false, no required params,
    /// `not_applicable` None, no embedded YAML (`embedded_text` None), no cases (`embedded_cases` None) and `position`
    /// 0. The scope is not rebuilt when a caller later sets `paths`, `exclude`, `kits` or `outside_kits`: the caller
    /// sets `scope` with `Scope::build` for that.
    pub fn new(id: &str, engine: Engine, check: &str) -> Rule {
        Rule {
            id: id.to_string(),
            engine,
            kind: Kind::Quality,
            check: check.to_string(),
            config: None,
            paths: Vec::new(),
            exclude: Vec::new(),
            kits: Vec::new(),
            outside_kits: Vec::new(),
            severity: Severity::Error,
            fix: String::new(),
            params: toml::Table::new(),
            scope: Scope { include: None, exclude: GlobSet::empty(), kits: None },
            enforces: None,
            leaves: None,
            only_with_base: false,
            required_params: Vec::new(),
            not_applicable: None,
            embedded_text: None,
            embedded_cases: None,
            position: 0,
        }
    }
}

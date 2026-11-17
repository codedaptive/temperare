//! `Rule`: one `[[rule]]` of the rule file.

use super::Scope;
use crate::{Engine, Kind, Severity};

/// A rule as read from the rule file, with its compiled scope. Empty `paths` covers every file; empty `kits` sets no
/// kit limit; `kind` is Quality and `severity` Error when the rule file gives none.
#[derive(Debug, Clone)]
pub struct Rule {
    pub id: String,
    pub engine: Engine,
    pub kind: Kind,
    /// What the engine checks: a lint name or a file relative to the rule file's folder.
    pub check: String,
    pub config: Option<String>,
    pub paths: Vec<String>,
    pub exclude: Vec<String>,
    pub kits: Vec<String>,
    pub severity: Severity,
    pub fix: String,
    pub params: toml::Table,
    pub scope: Scope,
}

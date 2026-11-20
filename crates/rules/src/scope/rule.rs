//! `Rule`: one rule, as a `[[rule]]` of the rule file or built by `Rule::new`.

use super::Scope;
use crate::{Engine, Kind, Severity};

/// A rule, as read from the rule file or built by `Rule::new`, with its compiled scope. Empty `paths` covers every
/// file; empty `kits` sets no kit limit; empty `outside_kits` excludes no kit; `kind` is Quality and `severity` Error
/// when the rule file gives none.
#[derive(Debug, Clone)]
pub struct Rule {
    pub id: String,
    pub engine: Engine,
    pub kind: Kind,
    /// What the engine checks: a lint name or a file relative to the rule file's folder.
    pub check: String,
    pub config: Option<String>,
    pub paths: Vec<String>,
    /// The user's exclude globs; the compiled scope also excludes the outside kits' paths.
    pub exclude: Vec<String>,
    pub kits: Vec<String>,
    /// Kit names whose files the rule never covers; its compiled scope excludes their paths.
    pub outside_kits: Vec<String>,
    pub severity: Severity,
    pub fix: String,
    pub params: toml::Table,
    pub scope: Scope,
}

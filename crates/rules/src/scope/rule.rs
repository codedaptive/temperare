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
    /// The house-rule line or practice the rule enforces.
    pub enforces: Option<String>,
    /// What the rule's check does not cover.
    pub leaves: Option<String>,
    /// True when the rule reports only what a change adds.
    pub only_with_base: bool,
    /// Params the repository must give through an `[[override]]`; only a baseline set's rules have them.
    pub required_params: Vec<String>,
    /// Why the rule is not applicable (e.g. a required param not given); None when it applies.
    pub not_applicable: Option<String>,
}

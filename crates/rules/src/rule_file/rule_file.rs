//! `RuleFile`: a rule file, read and checked or built empty by `RuleFile::new`.

use crate::{Exception, Kit, Rule};
use std::path::PathBuf;

/// The rule file's kits, rules, exceptions and baseline sets; every path in it is relative to `folder`, the rule file's folder.
#[derive(Debug, Clone)]
pub struct RuleFile {
    pub folder: PathBuf,
    /// The kits under `[kits]`, in file order.
    pub kits: Vec<Kit>,
    /// The top-level `owner` as written; None when absent, blank or not a string.
    pub owner: Option<String>,
    /// The rules that apply: the taken baseline sets' rules, with the overrides applied, then the file's own.
    pub rules: Vec<Rule>,
    pub exceptions: Vec<Exception>,
    /// The baseline sets the file takes, as "name@version" in file order; empty without a `baseline` key.
    pub baseline: Vec<String>,
    /// The ids the taken sets list as retired, in taken order.
    pub retired: Vec<String>,
    /// The baseline rules that are not applicable, each with its reason in `rule.not_applicable`. The parse keeps
    /// them out of `rules`.
    pub not_applicable_rules: Vec<Rule>,
}

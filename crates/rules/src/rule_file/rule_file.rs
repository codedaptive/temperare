//! `RuleFile`: a rule file, read and checked or built empty by `RuleFile::new`.

use crate::{Exception, Kit, Rule};
use std::path::PathBuf;

/// The rule file's kits, rules and exceptions; every path in it is relative to `folder`, the rule file's folder.
#[derive(Debug, Clone)]
pub struct RuleFile {
    pub folder: PathBuf,
    /// The kits under `[kits]`, in file order.
    pub kits: Vec<Kit>,
    /// The top-level `owner` as written; None when absent, blank or not a string.
    pub owner: Option<String>,
    pub rules: Vec<Rule>,
    pub exceptions: Vec<Exception>,
}

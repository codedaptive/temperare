//! `RuleFile`: a read and checked rule file.

use crate::{Exception, Rule};
use std::collections::BTreeMap;
use std::path::PathBuf;

/// The rule file's kits, rules and exceptions; every path in it is relative to `folder`, the rule file's folder.
#[derive(Debug, Clone)]
pub struct RuleFile {
    pub folder: PathBuf,
    /// Each kit's name and its path globs.
    pub kits: BTreeMap<String, Vec<String>>,
    pub rules: Vec<Rule>,
    pub exceptions: Vec<Exception>,
}

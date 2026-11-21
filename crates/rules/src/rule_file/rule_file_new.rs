//! `RuleFile::new`: an empty rule file built in code.

use super::RuleFile;
use std::path::PathBuf;

impl RuleFile {
    /// A rule file whose paths are relative to `folder`, with no kits, no owner, no rules, no exceptions, no baseline
    /// sets taken, no retired ids and no not-applicable rules. A caller fills the fields it needs on the result.
    pub fn new(folder: PathBuf) -> RuleFile {
        RuleFile {
            folder,
            kits: Vec::new(),
            owner: None,
            rules: Vec::new(),
            exceptions: Vec::new(),
            baseline: Vec::new(),
            retired: Vec::new(),
            not_applicable_rules: Vec::new(),
        }
    }
}

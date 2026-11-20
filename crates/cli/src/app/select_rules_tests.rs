//! Unit tests for select_rules.rs, on a rule file parsed from text with one quality and one security rule and an exception.

use super::select_rules::select_rules;
use rules::{Kind, RuleFile};
use std::path::{Path, PathBuf};

const TEXT: &str = "version = 1
owner = \"core\"
[[rule]]
id = \"q\"
engine = \"ruff\"
check = \"E501\"
fix = \"Wrap the line.\"
[[rule]]
id = \"s\"
engine = \"ast-grep\"
kind = \"security\"
check = \"no-eval\"
fix = \"Do not eval.\"
[[exception]]
rule = \"q\"
path = \"old.py\"
reason = \"legacy\"
approved = \"Bob, 2026-11-17\"
";

fn file() -> RuleFile {
    RuleFile::parse(TEXT, Path::new(".")).expect("valid rule file")
}

fn ids(rf: &RuleFile) -> Vec<&str> {
    rf.rules.iter().map(|r| r.id.as_str()).collect()
}

#[test]
fn no_kind_keeps_every_rule() {
    assert_eq!(ids(&select_rules(&file(), None)), vec!["q", "s"]);
}

#[test]
fn security_keeps_the_security_rule_alone() {
    assert_eq!(ids(&select_rules(&file(), Some(Kind::Security))), vec!["s"]);
}

#[test]
fn quality_keeps_the_quality_rule_alone() {
    assert_eq!(ids(&select_rules(&file(), Some(Kind::Quality))), vec!["q"]);
}

#[test]
fn a_kind_keeps_the_folder_owner_and_exceptions() {
    let selected = select_rules(&file(), Some(Kind::Security));
    assert_eq!(selected.folder, PathBuf::from("."));
    assert_eq!(selected.owner.as_deref(), Some("core"));
    let exceptions: Vec<(&str, &str)> =
        selected.exceptions.iter().map(|e| (e.rule.as_str(), e.path.as_str())).collect();
    assert_eq!(exceptions, vec![("q", "old.py")]);
}

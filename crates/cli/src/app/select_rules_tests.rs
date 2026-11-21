//! Unit tests for select_rules.rs, on a rule file parsed from text with one quality and one security rule and an exception,
//! plus three not-applicable rules pushed in code: security, quality, security.

use super::select_rules::select_rules;
use rules::{Engine, Kind, Rule, RuleFile};
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
expires = \"2099-12-31\"
";

fn file() -> RuleFile {
    let mut rf = RuleFile::parse(TEXT, Path::new(".")).expect("valid rule file");
    rf.not_applicable_rules.push(not_applicable("na-s1", Kind::Security));
    rf.not_applicable_rules.push(not_applicable("na-q", Kind::Quality));
    rf.not_applicable_rules.push(not_applicable("na-s2", Kind::Security));
    rf
}

fn not_applicable(id: &str, kind: Kind) -> Rule {
    let mut rule = Rule::new(id, Engine::AstGrep, "no-eval");
    rule.kind = kind;
    rule.not_applicable = Some("missing params".to_string());
    rule
}

fn na_ids(rf: &RuleFile) -> Vec<&str> {
    rf.not_applicable_rules.iter().map(|r| r.id.as_str()).collect()
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

#[test]
fn security_lists_only_the_security_not_applicable_rules_in_order() {
    assert_eq!(na_ids(&select_rules(&file(), Some(Kind::Security))), vec!["na-s1", "na-s2"]);
}

#[test]
fn quality_lists_only_the_quality_not_applicable_rule() {
    assert_eq!(na_ids(&select_rules(&file(), Some(Kind::Quality))), vec!["na-q"]);
}

#[test]
fn no_kind_keeps_both_lists_whole() {
    let selected = select_rules(&file(), None);
    assert_eq!(ids(&selected), vec!["q", "s"]);
    assert_eq!(na_ids(&selected), vec!["na-s1", "na-q", "na-s2"]);
}

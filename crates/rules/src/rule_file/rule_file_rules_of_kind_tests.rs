//! Unit tests for rule_file_rules_of_kind.rs. The RuleFile is built by `RuleFile::parse` on in-memory text.

use crate::{Kind, RuleFile};
use std::path::Path;

/// Five rules, quality and security interleaved; `a` gives no kind (so Quality). A security rule needs the owner.
const TEXT: &str = "version = 1
owner = \"core\"
[[rule]]
id = \"a\"
engine = \"clippy\"
check = \"c\"
fix = \"f\"
[[rule]]
id = \"s1\"
engine = \"tessera\"
kind = \"security\"
check = \"c\"
fix = \"f\"
[[rule]]
id = \"q\"
engine = \"ruff\"
kind = \"quality\"
check = \"c\"
fix = \"f\"
[[rule]]
id = \"s0\"
engine = \"ast-grep\"
kind = \"security\"
check = \"c\"
fix = \"f\"
[[rule]]
id = \"b\"
engine = \"swiftlint\"
check = \"c\"
fix = \"f\"
";

fn ids(file: &RuleFile, kind: Kind) -> Vec<&str> {
    file.rules_of_kind(kind).iter().map(|rule| rule.id.as_str()).collect()
}

#[test]
fn quality_rules_in_file_order() {
    let file = RuleFile::parse(TEXT, Path::new(".")).expect("valid");
    assert_eq!(ids(&file, Kind::Quality), vec!["a", "q", "b"]);
}

#[test]
fn security_rules_in_file_order() {
    let file = RuleFile::parse(TEXT, Path::new(".")).expect("valid");
    assert_eq!(ids(&file, Kind::Security), vec!["s1", "s0"]);
}

#[test]
fn no_rule_of_the_kind_is_empty() {
    let file = RuleFile::parse("version = 1\n[[rule]]\nid = \"a\"\nengine = \"ruff\"\ncheck = \"c\"\nfix = \"f\"\n", Path::new("."))
        .expect("valid");
    assert!(file.rules_of_kind(Kind::Security).is_empty());
}

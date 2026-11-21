//! ZUT for rule_file_exceptions_in_force.rs.

use crate::{Exception, RelPath, RuleFile};
use std::path::PathBuf;

fn exception(rule: &str, path: &str) -> Exception {
    Exception {
        rule: rule.to_string(),
        path: RelPath::new(path),
        what: None,
        expires: None,
        reason: "legacy".to_string(),
        approved: "Bob, 2026-11-17".to_string(),
    }
}

fn file(exceptions: Vec<Exception>) -> RuleFile {
    let mut f = RuleFile::new(PathBuf::from("."));
    f.exceptions = exceptions;
    f
}

#[test]
fn every_exception_is_returned_in_file_order() {
    let rules = file(vec![exception("b-rule", "z.rs"), exception("a-rule", "a.rs"), exception("b-rule", "m.rs")]);
    let got: Vec<(&str, &str)> =
        rules.exceptions_in_force().iter().map(|e| (e.rule.as_str(), e.path.as_str())).collect();
    assert_eq!(got, vec![("b-rule", "z.rs"), ("a-rule", "a.rs"), ("b-rule", "m.rs")]);
}

#[test]
fn a_file_without_exceptions_has_none_in_force() {
    assert_eq!(file(Vec::new()).exceptions_in_force().len(), 0);
}

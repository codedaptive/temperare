//! ZUT for rule_file_exceptions_expired.rs.

use crate::{Date, Exception, RuleFile};
use std::path::PathBuf;

fn file(exceptions: Vec<Exception>) -> RuleFile {
    let mut f = RuleFile::new(PathBuf::from("."));
    f.exceptions = exceptions;
    f
}

fn dated(rule: &str, expires: Option<Date>) -> Exception {
    Exception { expires, ..Exception::new(rule, "src/a.rs", "legacy", "Bob, 2026-11-17") }
}

#[test]
fn the_exceptions_past_their_date_are_listed_in_file_order() {
    let a = dated("a-rule", Date::new(2026, 11, 20));
    let b = dated("b-rule", None);
    let c = dated("c-rule", Date::new(2026, 11, 21));
    let d = dated("d-rule", Date::new(2026, 1, 1));
    let full = file(vec![a.clone(), b, c, d.clone()]);
    let empty = file(Vec::new());
    let rows: [(&str, &RuleFile, Vec<&Exception>); 2] =
        [("mixed file", &full, vec![&a, &d]), ("empty file", &empty, Vec::new())];
    for (name, rules, expected) in rows {
        assert_eq!(rules.exceptions_expired(Date::new(2026, 11, 21).unwrap()), expected, "{name}");
    }
}

//! ZUT for rule_file_exception_on.rs: today the answer is exception_for's on every day.

use crate::{Date, Engine, Exception, Finding, Kind, RelPath, RuleFile, Severity};
use std::path::PathBuf;

fn file() -> RuleFile {
    let mut file = RuleFile::new(PathBuf::from("."));
    file.exceptions = vec![Exception::new("no-unwrap", "src/a.rs", "startup only", "Bob, 2026-11-17")];
    file
}

fn finding(rule: &str, path: &str) -> Finding {
    Finding {
        rule: rule.to_string(),
        engine: Engine::Clippy,
        kind: Kind::Quality,
        severity: Severity::Error,
        owner: None,
        path: RelPath::new(path),
        line: 3,
        col: Some(9),
        what: "used `unwrap()` on a Result".to_string(),
        fix: "Return a typed error.".to_string(),
    }
}

fn day(year: u16, month: u8, day: u8) -> Date {
    Date::new(year, month, day).unwrap()
}

#[test]
fn a_matching_finding_gets_the_same_exception_on_any_day() {
    let rules = file();
    let hit = finding("no-unwrap", "src/a.rs");
    let expected = Exception::new("no-unwrap", "src/a.rs", "startup only", "Bob, 2026-11-17");
    assert_eq!(rules.exception_on(&hit, day(2020, 1, 1)), Some(&expected));
    assert_eq!(rules.exception_on(&hit, day(2099, 12, 31)), Some(&expected));
    assert_eq!(rules.exception_on(&hit, day(2020, 1, 1)), rules.exception_for(&hit));
}

#[test]
fn a_finding_no_exception_covers_gets_none() {
    let rules = file();
    assert_eq!(rules.exception_on(&finding("no-expect", "src/a.rs"), day(2020, 1, 1)), None);
    assert_eq!(rules.exception_on(&finding("no-unwrap", "src/b.rs"), day(2099, 12, 31)), None);
}

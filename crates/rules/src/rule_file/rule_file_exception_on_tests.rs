//! Unit tests for rule_file_exception_on.rs: the first covering exception that still holds on the day.

use crate::{Date, Engine, Exception, Finding, Kind, RelPath, RuleFile, Severity};
use std::path::PathBuf;

fn file(exceptions: Vec<Exception>) -> RuleFile {
    let mut file = RuleFile::new(PathBuf::from("."));
    file.exceptions = exceptions;
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
        line: Some(3),
        col: Some(9),
        what: "used `unwrap()` on a Result".to_string(),
        fix: "Return a typed error.".to_string(),
    }
}

fn day(year: u16, month: u8, day: u8) -> Date {
    Date::new(year, month, day).unwrap()
}

fn dated(reason: &str, expires: Option<Date>) -> Exception {
    Exception { expires, ..Exception::new("no-unwrap", "src/a.rs", reason, "Bob, 2026-11-17") }
}

#[test]
fn a_covering_exception_holds_through_its_expires_day() {
    let ends = dated("startup only", Date::new(2026, 12, 31));
    let open = dated("startup only", None);
    let rows: [(&str, &Exception, Date, Option<&Exception>); 4] = [
        ("day before expires", &ends, day(2026, 12, 30), Some(&ends)),
        ("expires day", &ends, day(2026, 12, 31), Some(&ends)),
        ("day after expires", &ends, day(2027, 1, 1), None),
        ("no expires", &open, day(2099, 12, 31), Some(&open)),
    ];
    for (name, exception, on, expected) in rows {
        let rules = file(vec![exception.clone()]);
        assert_eq!(rules.exception_on(&finding("no-unwrap", "src/a.rs"), on), expected, "{name}");
    }
}

#[test]
fn an_expired_exception_is_passed_over_for_a_later_one_that_holds() {
    let rules = file(vec![dated("startup only", Date::new(2026, 1, 1)), dated("until the rewrite", None)]);
    let expected = dated("until the rewrite", None);
    let rows: [(&str, Finding, Option<&Exception>); 2] = [
        ("later one holds", finding("no-unwrap", "src/a.rs"), Some(&expected)),
        ("none covers", finding("no-unwrap", "src/b.rs"), None),
    ];
    for (name, hit, want) in rows {
        assert_eq!(rules.exception_on(&hit, day(2026, 11, 21)), want, "{name}");
    }
}

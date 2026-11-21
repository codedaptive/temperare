//! ZUT for exception_loosenings.rs, on rule files built here: added, widened by dropping `what`, narrowed, and an
//! `expires` moved later, dropped, moved earlier or kept against one or two covering base exceptions.

use super::exception_loosenings::exception_loosenings;
use super::{Loosening, LooseningKind};
use crate::{Date, Exception, RuleFile};
use std::path::PathBuf;

fn exception(what: Option<&str>) -> Exception {
    dated(what, None)
}

fn dated(what: Option<&str>, expires: Option<Date>) -> Exception {
    Exception { expires, what: what.map(String::from), ..Exception::new("r1", "src/a.py", "legacy", "bob") }
}

fn file(exceptions: Vec<Exception>) -> RuleFile {
    let mut file = RuleFile::new(PathBuf::from("."));
    file.exceptions = exceptions;
    file
}

fn added(after: &str) -> Loosening {
    Loosening { kind: LooseningKind::ExceptionAdded, about: "r1".to_string(), before: None, after: Some(after.to_string()) }
}

fn later(before: &str, after: &str) -> Loosening {
    Loosening {
        kind: LooseningKind::ExpiryLater,
        about: "r1".to_string(),
        before: Some(before.to_string()),
        after: Some(after.to_string()),
    }
}

fn dropped(before: &str) -> Loosening {
    Loosening { kind: LooseningKind::ExpiryDropped, about: "r1".to_string(), before: Some(before.to_string()), after: None }
}

#[test]
fn exceptions_no_base_exception_covers_are_found() {
    let jan26 = Date::new(2026, 1, 1);
    let mar26 = Date::new(2026, 3, 1);
    let jun26 = Date::new(2026, 6, 1);
    let sep26 = Date::new(2026, 9, 1);
    let jan27 = Date::new(2027, 1, 1);
    let rows = [
        ("added", vec![], vec![exception(None)], vec![added("src/a.py")]),
        ("added with what", vec![], vec![exception(Some("eval"))], vec![added("src/a.py (eval)")]),
        ("what dropped", vec![exception(Some("eval"))], vec![exception(None)], vec![added("src/a.py")]),
        ("narrowed", vec![exception(None)], vec![exception(Some("eval"))], vec![]),
        ("later", vec![dated(None, jan26)], vec![dated(None, jan27)], vec![later("2026-01-01", "2027-01-01")]),
        ("dropped", vec![dated(None, jan26)], vec![dated(None, None)], vec![dropped("2026-01-01")]),
        ("earlier", vec![dated(None, jan27)], vec![dated(None, jan26)], vec![]),
        ("same date", vec![dated(None, jan26)], vec![dated(None, jan26)], vec![]),
        ("base undated, now dated", vec![dated(None, None)], vec![dated(None, jan26)], vec![]),
        ("one coverer undated, now undated", vec![dated(None, jan26), dated(None, None)], vec![dated(None, None)], vec![]),
        ("within the latest coverer", vec![dated(None, jan26), dated(None, jun26)], vec![dated(None, mar26)], vec![]),
        (
            "past the latest coverer",
            vec![dated(None, jan26), dated(None, jun26)],
            vec![dated(None, sep26)],
            vec![later("2026-06-01", "2026-09-01")],
        ),
    ];
    for (case, base, now, expected) in rows {
        assert_eq!(exception_loosenings(&file(base), &file(now)), expected, "{case}");
    }
}

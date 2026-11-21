//! Unit tests for split_exceptions.rs: which exceptions are in force and which expired on a pinned day, in file order.

use super::split_exceptions::split_exceptions;
use rules::{Date, RuleFile};
use std::path::Path;

/// (path, expires) in file order. On 2026-11-21 in force: f (expires that day), d (none), e (later); expired: b, a, c
/// (each before it). Neither list is sorted by path or by expires, either way, wherever a missing date goes.
const EXCEPTIONS: [(&str, Option<&str>); 6] = [
    ("f.rs", Some("2026-11-21")),
    ("b.rs", Some("2026-06-01")),
    ("d.rs", None),
    ("a.rs", Some("2026-11-20")),
    ("e.rs", Some("2026-12-01")),
    ("c.rs", Some("2026-01-02")),
];

fn rule_file() -> RuleFile {
    let mut text = String::from("version = 1\n[[rule]]\nid = \"r\"\nengine = \"clippy\"\ncheck = \"c\"\nfix = \"f\"\n");
    for (path, expires) in EXCEPTIONS {
        text.push_str(&format!("[[exception]]\nrule = \"r\"\npath = \"{path}\"\nreason = \"x\"\napproved = \"bob\"\n"));
        if let Some(date) = expires {
            text.push_str(&format!("expires = \"{date}\"\n"));
        }
    }
    RuleFile::parse(&text, Path::new(".")).unwrap()
}

#[test]
fn in_force_and_expired_on_the_day_in_file_order() {
    let rf = rule_file();
    let (in_force, expired) = split_exceptions(&rf, Date::new(2026, 11, 21).unwrap());
    let paths = |list: Vec<&rules::Exception>| list.iter().map(|e| e.path.as_str().to_string()).collect::<Vec<_>>();
    assert_eq!(paths(in_force), ["f.rs", "d.rs", "e.rs"]);
    assert_eq!(paths(expired), ["b.rs", "a.rs", "c.rs"]);
}

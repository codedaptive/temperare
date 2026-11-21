//! Unit tests for set_loosenings.rs, on rule files built here: a set dropped and a set taken at an older version.

use super::set_loosenings::set_loosenings;
use super::{Loosening, LooseningKind};
use crate::RuleFile;
use std::path::PathBuf;

fn file(baseline: &[&str]) -> RuleFile {
    let mut file = RuleFile::new(PathBuf::from("."));
    file.baseline = baseline.iter().map(|entry| entry.to_string()).collect();
    file
}

fn found(kind: LooseningKind, before: &str, after: Option<&str>) -> Loosening {
    Loosening { kind, about: "standard".to_string(), before: Some(before.to_string()), after: after.map(String::from) }
}

#[test]
fn sets_dropped_or_older_are_found() {
    let rows = [
        ("set dropped", file(&[]), vec![found(LooseningKind::SetDropped, "2", None)]),
        ("set older", file(&["standard@1"]), vec![found(LooseningKind::SetOlder, "2", Some("1"))]),
    ];
    for (case, now, expected) in rows {
        assert_eq!(set_loosenings(&file(&["standard@2"]), &now), expected, "{case}");
    }
}

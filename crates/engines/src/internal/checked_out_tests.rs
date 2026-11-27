//! Unit test for checked_out: only checked-out tracked paths, in their original order.

use crate::checked_out::checked_out;
use ccvcslib::{RevisionEntryKind, TrackedEntry, WorkTreeState};
use std::path::PathBuf;

#[test]
fn checked_out_cases() {
    use RevisionEntryKind::{ExecutableFile, File, Link, NestedRepository};
    use WorkTreeState::{CheckedOut, LeftOut, Missing};
    let cases = [
        ("mixed", vec![("z.sh", ExecutableFile, CheckedOut), ("left", File, LeftOut), ("missing", Link, Missing), ("a", NestedRepository, CheckedOut)], vec!["z.sh", "a"]),
        ("empty", vec![], vec![]),
    ];
    let mut wrong = Vec::new();
    for (name, entries, expected) in cases {
        let entries = entries.into_iter().map(|(path, kind, state)| TrackedEntry { path: path.into(), kind, state }).collect();
        let expected: Vec<PathBuf> = expected.into_iter().map(PathBuf::from).collect();
        let got = checked_out(entries);
        if got != expected {
            wrong.push(format!("{name}: got {got:?}, expected {expected:?}"));
        }
    }
    assert!(wrong.is_empty(), "{wrong:#?}");
}

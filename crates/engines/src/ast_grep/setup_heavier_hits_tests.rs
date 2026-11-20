//! ZUT for setup_heavier_hits.rs, on ast-grep 0.51.1's real matches (fixtures/count_rust.jsonl; its files are
//! described in count_file_tests.rs). By hand: src/a.rs has 10 lines of setup and 7 of tests, src/c.rs 1 and 1,
//! src/d.rs 1 and 3, and src/b.rs has no test.

use std::collections::BTreeMap;
use std::path::Path;

use procio::Output;
use rules::{RelPath, Scope};

use super::{parse_count_matches, setup_heavier_hits, CountMatch};
use crate::Hit;

const COUNT_RUST: &[u8] = include_bytes!("fixtures/count_rust.jsonl");

fn added(files: &[(&str, u32)]) -> BTreeMap<RelPath, u32> {
    files.iter().map(|(path, line)| (RelPath::new(path), *line)).collect()
}

fn hit(path: &str, line: u32, what: &str) -> Hit {
    Hit { path: RelPath::new(path), line: Some(line), col: None, what: what.to_string() }
}

#[test]
fn files_the_change_adds_to_with_more_setup_than_tests_are_hits() {
    let out = Output { code: Some(0), stdout: COUNT_RUST.to_vec(), stderr: Vec::new() };
    let fixture = parse_count_matches(&out).unwrap();
    // The fixture plus a copy of src/a.rs's matches as tests/common/mod.rs, a shared setup file.
    let mut with_shared = fixture.clone();
    with_shared.extend(fixture.iter().filter(|found| found.file == "src/a.rs").map(|found| CountMatch {
        file: "tests/common/mod.rs".into(),
        ..found.clone()
    }));
    let shared = Scope::build(&["tests/common/**".into()], &[], None).unwrap();
    let every = added(&[("src/a.rs", 5), ("src/b.rs", 1), ("src/c.rs", 2), ("src/d.rs", 3), ("tests/common/mod.rs", 4)]);
    let a_hit = hit("src/a.rs", 5, "10 lines of setup, 7 lines of tests");
    let w = Path::new("/w");
    // (row, matches, folder, added, shared_setup, expected)
    let rows: [(&str, &[CountMatch], &Path, BTreeMap<RelPath, u32>, Option<&Scope>, Vec<Hit>); 5] = [
        // c.rs (equal) and d.rs (under) are not reported; b.rs has no test.
        ("setup over tests, at the first added line", &fixture, w, every.clone(), None, vec![a_hit.clone()]),
        ("a file not in added", &fixture, w, added(&[("src/c.rs", 2), ("src/d.rs", 3)]), None, vec![]),
        ("shared setup exempt", &with_shared, w, every.clone(), Some(&shared), vec![a_hit.clone()]),
        (
            "hits in path order",
            &with_shared,
            w,
            every,
            None,
            vec![a_hit, hit("tests/common/mod.rs", 4, "10 lines of setup, 7 lines of tests")],
        ),
        (
            "paths relative to the folder",
            &fixture,
            Path::new("/w/src"),
            added(&[("a.rs", 9)]),
            None,
            vec![hit("a.rs", 9, "10 lines of setup, 7 lines of tests")],
        ),
    ];
    for (row, matches, folder, added, shared_setup, expected) in rows {
        assert_eq!(setup_heavier_hits(matches, w, folder, &added, shared_setup), expected, "{row}");
    }
}

#[test]
fn count_edges_without_tests_are_not_hits() {
    let out = Output { code: Some(0), stdout: include_bytes!("fixtures/count_edges_rust.jsonl").to_vec(), stderr: Vec::new() };
    let all = parse_count_matches(&out).unwrap();
    let w = Path::new("/w");
    // Empty: no matches. Whitespace: one empty root, no test; neither has a count to report.
    let rows = [("empty.rs", 0), ("whitespace.rs", 1)];
    let mut failed = Vec::new();
    for (file, match_count) in rows {
        let matches: Vec<CountMatch> = all.iter().filter(|found| found.file == file).cloned().collect();
        if std::panic::catch_unwind(|| {
            assert_eq!(matches.len(), match_count, "{file}");
            assert_eq!(setup_heavier_hits(&matches, w, w, &added(&[(file, 1)]), None), Vec::<Hit>::new(), "{file}");
        }).is_err() {
            failed.push(file);
        }
    }
    assert!(failed.is_empty(), "failed rows: {failed:?}");
}

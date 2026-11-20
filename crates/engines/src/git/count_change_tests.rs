//! ZUT for count_change.rs.

use super::{count_change, Counts, TestRatio};
use rules::{Engine, RelPath, Rule, Scope};
use std::ops::Range;

fn strings(globs: &[&str]) -> Vec<String> {
    globs.iter().map(|g| g.to_string()).collect()
}

fn rule(paths: &[&str]) -> Rule {
    let mut rule = Rule::new("tm-house-test-ratio", Engine::Git, "test_ratio");
    rule.paths = strings(paths);
    rule.scope = Scope::build(&rule.paths, &[], None).unwrap();
    rule
}

fn ratio() -> TestRatio {
    let test_paths = Scope::build(&strings(&["**/tests/**", "**/*_tests.rs"]), &[], None).unwrap();
    TestRatio { max_ratio: 1.5, floor: 40, test_paths }
}

fn files(given: &[(&str, &[Range<u32>])]) -> Vec<(RelPath, Vec<Range<u32>>)> {
    given.iter().map(|(path, ranges)| (RelPath::new(path), ranges.to_vec())).collect()
}

fn top(path: &str, line: u32) -> Option<(RelPath, u32)> {
    Some((RelPath::new(path), line))
}

#[test]
fn counts_a_change() {
    let cases: [(&str, &[&str], Vec<(&str, &[Range<u32>])>, Counts); 5] = [
        (
            "mixed",
            &["**/*.rs"],
            vec![
                ("src/a.rs", &[3..8, 20..22]),
                ("src/a_tests.rs", &[10..30, 4..6]),
                ("tests/b.rs", &[1..5]),
            ],
            Counts { test: 26, code: 7, top_test: top("src/a_tests.rs", 4) },
        ),
        (
            "out of scope",
            &["src/**"],
            vec![("src/a.rs", &[1..4]), ("tests/b.rs", &[1..90]), ("src/a_tests.rs", &[2..4])],
            Counts { test: 2, code: 3, top_test: top("src/a_tests.rs", 2) },
        ),
        (
            "no ranges is not top",
            &["**/*.rs"],
            vec![("a/tests/x.rs", &[]), ("src/a.rs", &[1..2])],
            Counts { test: 0, code: 1, top_test: None },
        ),
        (
            "tie to the smaller path",
            &["**/*.rs"],
            vec![("z/tests/b.rs", &[5..15]), ("a/tests/b.rs", &[7..17]), ("m/tests/b.rs", &[1..3])],
            Counts { test: 22, code: 0, top_test: top("a/tests/b.rs", 7) },
        ),
        (
            "untracked file",
            &["**/*.rs"],
            vec![("tests/new.rs", &[1..51]), ("src/a.rs", &[9..12])],
            Counts { test: 50, code: 3, top_test: top("tests/new.rs", 1) },
        ),
    ];
    for (name, paths, given, want) in cases {
        assert_eq!(count_change(&rule(paths), &ratio(), &files(&given)), want, "{name}");
    }
}

//! ZUT for count_file.rs, on ast-grep 0.51.1's real matches (fixtures/count_rust.jsonl and, saved the same way with
//! the same two rules, fixtures/count_edges_rust.jsonl: last.rs, whitespace.rs, only.rs; empty.rs gives no match, so
//! it is absent from the fixture) and on hand-made ones.
//!
//! The fixture is `ast-grep scan --rule rule.yml --json=stream src`, where rule.yml is two rules split by `---`:
//! `rust-root` (`kind: source_file`) and `rust-test` (`kind: function_item` that `follows` an `attribute_item` that
//! `has` an `attribute` with `regex: ^test$`). The files, by line (0-based):
//! - src/a.rs, 21 lines and a trailing newline: 0 `// lead comment`, 1 `use std::fmt;`, 2 blank, 3-5 `fn helper`,
//!   6 blank, 7 `#[test]`, 8-12 `fn one` (line 10 blank), 13 blank, 14 `mod inner {`, 15 `#[test]`, 16-18 `fn two`,
//!   19 `}`, 20 `// trail comment`. Its two tests print before its root.
//! - src/b.rs: two functions, no test, no trailing newline.
//! - src/c.rs: `#[test]` then `fn t() {}`, with a trailing newline.
//! - src/d.rs: two blank lines, `#[test]`, then a three-line test, no trailing newline.

use procio::Output;

use super::{count_file, parse_count_matches, CountMatch, FileCount};

const COUNT_RUST: &[u8] = include_bytes!("fixtures/count_rust.jsonl");

fn hand(text: &str, start_byte: usize, end_byte: usize, start_line: u32, end_line: u32, end_column: u32) -> CountMatch {
    CountMatch { file: "x.rs".into(), text: text.into(), start_byte, end_byte, start_line, end_line, end_column }
}

#[test]
fn a_files_lines_are_setup_or_tests() {
    let out = Output { code: Some(0), stdout: COUNT_RUST.to_vec(), stderr: Vec::new() };
    let all = parse_count_matches(&out).unwrap();
    let of = |file: &str| -> Vec<CountMatch> { all.iter().filter(|found| found.file == file).cloned().collect() };
    // Column 0 of line 4, past the newline that ends line 3: line 4 (`b`) is outside the test.
    let column_zero = vec![hand("a\nfn t() {\nx\n}\nb\n", 0, 18, 0, 5, 0), hand("fn t() {\nx\n}\n", 2, 16, 1, 4, 0)];
    // (row, matches, expected), each expected counted by hand from the lines above.
    let rows: [(&str, Vec<CountMatch>, Option<FileCount>); 6] = [
        // setup: lines 0, 1, 3, 4, 5, 7, 14, 15, 19, 20 (comments and the #[test] lines count); tests: 8, 9, 11, 12,
        // 16, 17, 18 (blank line 10 inside `fn one` is not counted).
        ("a.rs, nested, root last", of("src/a.rs"), Some(FileCount { setup: 10, tests: 7 })),
        ("b.rs, no test", of("src/b.rs"), None),
        // setup: line 0 (#[test]); tests: line 1. The trailing newline makes no line.
        ("c.rs, equal", of("src/c.rs"), Some(FileCount { setup: 1, tests: 1 })),
        // The root starts at line 2 (the blank lines are outside it); setup: line 2; tests: lines 3, 4, 5.
        ("d.rs, no trailing newline", of("src/d.rs"), Some(FileCount { setup: 1, tests: 3 })),
        // setup: lines 0 (`a`) and 4 (`b`); tests: lines 1, 2, 3.
        ("test ending at column 0", column_zero, Some(FileCount { setup: 2, tests: 3 })),
        ("no matches", Vec::new(), None),
    ];
    for (row, matches, expected) in rows {
        let refs: Vec<&CountMatch> = matches.iter().collect();
        assert_eq!(count_file(&refs), expected, "{row}");
    }
}

#[test]
fn count_edges_per_file() {
    let stdout = include_bytes!("fixtures/count_edges_rust.jsonl").to_vec();
    let out = Output { code: Some(0), stdout, stderr: Vec::new() };
    let all = parse_count_matches(&out).unwrap();
    let of = |file: &str| -> Vec<CountMatch> { all.iter().filter(|found| found.file == file).cloned().collect() };
    let last = of("last.rs");
    let root = last.iter().find(|found| found.start_byte == 0).unwrap();
    // Seven lines, numbered 0..=6; end is at the start of the next line.
    assert_eq!((root.end_line, root.end_column), (7, 0));
    assert!(root.text.ends_with('\n'));
    let only = of("only.rs");
    assert_eq!((only[0].start_line, only[1].start_line), (1, 0));
    let whitespace = of("whitespace.rs");
    assert_eq!(whitespace.len(), 1);
    assert_eq!(whitespace[0].text, "");
    let python = "def test_x():\n    assert True";
    let identical = vec![hand(python, 0, 29, 0, 1, 15), hand(python, 0, 29, 0, 1, 15)];
    let rows = [
        // setup: 0 use, 1 helper, 3 attribute; tests: 4 function, 5 assertion, 6 closing brace.
        ("newline ending", last, Some(FileCount { setup: 3, tests: 3 })),
        // No match for an empty file; whitespace has one empty root and no test.
        ("empty", of("empty.rs"), None),
        ("whitespace", whitespace, None),
        // setup: 0 attribute; tests: 1 function, 2 assertion, 3 closing brace.
        ("test before root", only, Some(FileCount { setup: 1, tests: 3 })),
        // setup: none; tests: 0 function, 1 assertion. Whichever tied match is the root, the other covers every line.
        ("identical ranges", identical, Some(FileCount { setup: 0, tests: 2 })),
    ];
    let mut failed = Vec::new();
    for (row, matches, expected) in rows {
        let refs: Vec<&CountMatch> = matches.iter().collect();
        if std::panic::catch_unwind(|| assert_eq!(count_file(&refs), expected, "{row}")).is_err() {
            failed.push(row);
        }
    }
    assert!(failed.is_empty(), "failed rows: {failed:?}");
}

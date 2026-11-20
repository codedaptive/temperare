//! ZUT for parse_count_matches.rs, given ast-grep 0.51.1's real output captured once (fixtures/count_rust.jsonl:
//! `ast-grep scan --rule <yaml> --json=stream src` over four Rust files, exit 0; the YAML is count_file_tests.rs's),
//! the empty output of a clean run, and the stderr of a rule YAML ast-grep cannot parse (fixtures/).

use procio::Output;

use super::{parse_count_matches, CountMatch};

const COUNT_RUST: &[u8] = include_bytes!("fixtures/count_rust.jsonl");
const CLEAN: &[u8] = include_bytes!("fixtures/clean.jsonl");
const BROKEN_RULE_STDERR: &[u8] = include_bytes!("fixtures/broken_rule.stderr");

fn output(code: Option<i32>, stdout: &[u8], stderr: &[u8]) -> Output {
    Output { code, stdout: stdout.to_vec(), stderr: stderr.to_vec() }
}

#[test]
fn every_match_is_read_in_output_order() {
    let got = parse_count_matches(&output(Some(0), COUNT_RUST, b"")).unwrap();
    // The fixture's lines name these files in this order (read off the file); a.rs prints its tests before its root.
    let files: Vec<&str> = got.iter().map(|found| found.file.as_str()).collect();
    assert_eq!(files, ["src/c.rs", "src/c.rs", "src/d.rs", "src/d.rs", "src/b.rs", "src/a.rs", "src/a.rs", "src/a.rs"]);
    // The first two lines, field by field: c.rs's root ("#[test]\nfn t() {}\n", 18 bytes, ending at column 0 of the
    // line after the last) and its test function (`fn t() {}`, bytes 8..17 on line 1).
    let root = CountMatch {
        file: "src/c.rs".into(),
        text: "#[test]\nfn t() {}\n".into(),
        start_byte: 0,
        end_byte: 18,
        start_line: 0,
        end_line: 2,
        end_column: 0,
    };
    let test = CountMatch {
        file: "src/c.rs".into(),
        text: "fn t() {}".into(),
        start_byte: 8,
        end_byte: 17,
        start_line: 1,
        end_line: 1,
        end_column: 9,
    };
    assert_eq!(got[..2], [root, test]);
}

#[test]
fn output_that_is_not_an_answer_is_an_error() {
    // (row, output, whether it is an error, text the error must hold)
    let rows: [(&str, Output, bool, &str); 5] = [
        ("clean run", output(Some(0), CLEAN, b""), false, ""),
        ("rule ast-grep cannot read", output(Some(8), CLEAN, BROKEN_RULE_STDERR), true, "Error: Cannot parse rule"),
        ("killed run", output(None, b"", b""), true, "(exit None)"),
        ("line not json", output(Some(0), b"oops, not json\n", b""), true, "oops, not json"),
        ("not utf-8", output(Some(0), b"\xff\xfe\n", b""), true, "(exit Some(0))"),
    ];
    for (row, out, is_error, holds) in rows {
        match parse_count_matches(&out) {
            Ok(found) => assert!(!is_error && found.is_empty(), "{row}: {found:?}"),
            Err(error) => {
                assert!(is_error, "{row}: {error:?}");
                assert!(error.why.starts_with("ast-grep output is not JSON match lines"), "{row}: {}", error.why);
                assert!(error.why.contains(holds), "{row}: {}", error.why);
            }
        }
    }
}

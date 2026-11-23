//! ZUT for parse_rubocop.rs: real rubocop 1.92.3 reports (see fixtures/fixtures.txt) read into hits. The reports'
//! paths are relative, so any folder stands for the one rubocop ran in. The rows with a non-0-or-1 exit pair a
//! captured stdout with hand-built stderr, and the no-report row's stdout is hand-built, to show what the error quotes.

use super::parse_rubocop;
use crate::types::{Hit, ParseError};
use procio::Output;
use rules::RelPath;
use std::path::Path;

const OFFENSES: &[u8] = include_bytes!("fixtures/check_offenses.json");
const SYNTAX_MESSAGE: &str = "unexpected token $end\n(Using Ruby 2.7 parser; configure using `TargetRubyVersion` parameter, under `AllCops`)";

fn hit(path: &str, line: u32, col: u32, what: &str) -> Hit {
    Hit { path: RelPath::new(path), line: Some(line), col: Some(col), what: what.to_string() }
}

fn why(why: &str) -> Result<Vec<Hit>, ParseError> {
    Err(ParseError { why: why.to_string() })
}

#[test]
fn parse_rubocop_reads_the_checked_cops_offenses_or_refuses_the_output() {
    let unused = hit("example.rb", 4, 3, "Useless assignment to variable - `unused`.");
    let quiet = hit("example.rb", 5, 3, "Useless assignment to variable - `quiet`.");
    let double = hit("example.rb", 6, 8, "Prefer single-quoted strings when you don't need string interpolation or special symbols.");
    let left = hit("lib/other.rb", 4, 3, "Useless assignment to variable - `left`.");
    let cases: [(&str, Option<i32>, &[u8], &[u8], &str, &str, Result<Vec<Hit>, ParseError>); 10] = [
        ("one cop", Some(1), OFFENSES, b"", "Lint/UselessAssignment", "/w/rb", Ok(vec![unused.clone(), quiet.clone(), left.clone()])),
        ("the other cop", Some(1), OFFENSES, b"", "Style/StringLiterals", "/w/rb", Ok(vec![double.clone()])),
        ("both, in output order", Some(1), OFFENSES, b"", "Style/StringLiterals,Lint/UselessAssignment", "/w/rb", Ok(vec![unused, quiet, double, left])),
        ("a file outside the folder", Some(1), OFFENSES, b"", "Lint/UselessAssignment", "/w/rb/lib", Ok(vec![hit("other.rb", 4, 3, "Useless assignment to variable - `left`.")])),
        ("clean", Some(0), include_bytes!("fixtures/check_clean.json"), b"", "Lint/UselessAssignment", "/w/rb", Ok(vec![])),
        ("a file rubocop could not parse", Some(1), include_bytes!("fixtures/check_syntax.json"), b"", "Lint/UselessAssignment", "/w/rb",
            why(&format!("rubocop could not read broken.rb: {SYNTAX_MESSAGE}"))),
        ("Lint/Syntax as the check", Some(1), include_bytes!("fixtures/check_syntax.json"), b"", "Lint/Syntax", "/w/rb", Ok(vec![hit("broken.rb", 4, 12, SYNTAX_MESSAGE)])),
        ("exit 2 with a report", Some(2), include_bytes!("fixtures/check_unknown.json"), b"Error: bad config\n", "Lint/UselessAssignment", "/w/rb",
            why("rubocop exited 2, not 0 or 1: Error: bad config\n")),
        ("stopped by a signal", None, include_bytes!("fixtures/check_clean.json"), b"killed", "Lint/UselessAssignment", "/w/rb",
            why("rubocop was stopped by a signal, not 0 or 1: killed")),
        ("no report", Some(0), b"garbage", b"", "Lint/UselessAssignment", "/w/rb", why("rubocop printed no JSON report: garbage")),
    ];
    let mut failed = Vec::new();
    for (name, code, stdout, stderr, check, folder, want) in cases {
        let result = std::panic::catch_unwind(|| {
            let out = Output { code, stdout: stdout.to_vec(), stderr: stderr.to_vec() };
            assert_eq!(parse_rubocop(&out, check, Path::new("/w/rb"), Path::new(folder)), want, "{name}");
        });
        if result.is_err() { failed.push(name); }
    }
    assert!(failed.is_empty(), "failed rows: {failed:?}");
}

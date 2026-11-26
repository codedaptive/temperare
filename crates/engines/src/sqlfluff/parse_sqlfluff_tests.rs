//! ZUT for parse_sqlfluff.rs: real sqlfluff 4.5.0 reports (fixtures/, made as fixtures/fixtures.txt records, each run
//! in its own folder, here `/w`, and each row's Output built with that run's exit code) read into hits, and the
//! outputs that are not an answer.

use super::parse_sqlfluff;
use crate::types::{Hit, ParseError};
use procio::Output;
use rules::RelPath;
use std::path::Path;

const FINDINGS: &[u8] = include_bytes!("fixtures/findings.json");
const CLEAN: &[u8] = include_bytes!("fixtures/clean.json");
const UNPARSABLE: &[u8] = include_bytes!("fixtures/unparsable.json");
const WARNINGS: &[u8] = include_bytes!("fixtures/warnings.json");
/// A templating violation in the report's shape, written by hand: TMP is read as PRS is.
const TEMPLATED: &[u8] = br#"[{"filepath": "t.sql", "violations": [{"start_line_no": 3, "start_line_pos": 1, "code": "TMP", "description": "Undefined jinja template variable: 'x'", "name": "", "warning": false}]}]"#;
const FROM_WHAT: &str = "LT01: Expected only single space before 'FROM' keyword. Found '  '.";

fn hit(path: &str, line: u32, col: u32, what: &str) -> Hit {
    Hit { path: RelPath::new(path), line: Some(line), col: Some(col), what: what.to_string() }
}

fn err(why: &str) -> Result<Vec<Hit>, ParseError> {
    Err(ParseError { why: why.to_string() })
}

#[test]
fn parse_sqlfluff_cases() {
    let plus = "LT01: Expected only single space before binary operator '+'. Found '  '.";
    let naked = "LT01: Expected only single space before naked identifier. Found '  '.";
    let rows: Vec<(&str, i32, &[u8], &[u8], &str, Result<Vec<Hit>, ParseError>)> = vec![
        ("findings in two files", 1, FINDINGS, b"", "/w", Ok(vec![
            hit("a.sql", 1, 9, FROM_WHAT),
            hit("sub/b.sql", 2, 6, plus),
            hit("sub/b.sql", 2, 9, naked),
        ])),
        ("a folder below the cwd", 1, FINDINGS, b"", "/w/sub", Ok(vec![hit("b.sql", 2, 6, plus), hit("b.sql", 2, 9, naked)])),
        ("clean", 0, CLEAN, b"", "/w", Ok(vec![])),
        ("a config warning is a finding", 0, WARNINGS, b"", "/w", Ok(vec![hit("a.sql", 1, 9, FROM_WHAT)])),
        ("a parse error", 1, UNPARSABLE, b"", "/w", err("sqlfluff could not read a.sql, so it did not check it: line 1: PRS: Couldn't find closing bracket for opening bracket.")),
        ("a templating error", 1, TEMPLATED, b"", "/w", err("sqlfluff could not read t.sql, so it did not check it: line 3: TMP: Undefined jinja template variable: 'x'")),
        ("exit 2", 2, b"", b"User Error: No dialect was specified.", "/w", err("sqlfluff stopped without a report: User Error: No dialect was specified.")),
        // The unknown-dialect run in fixtures/fixtures.txt, written by hand: exit 1, stdout empty, and of its
        // traceback only the last line.
        ("no JSON on exit 1", 1, b"", b"KeyError: 'Unknown dialect'\n", "/w", err("sqlfluff printed no JSON report: KeyError: 'Unknown dialect'\n")),
    ];
    for (label, code, stdout, stderr, folder, expected) in rows {
        let out = Output { code: Some(code), stdout: stdout.to_vec(), stderr: stderr.to_vec() };
        assert_eq!(parse_sqlfluff(&out, "LT01", Path::new("/w"), Path::new(folder)), expected, "{label}");
    }
}

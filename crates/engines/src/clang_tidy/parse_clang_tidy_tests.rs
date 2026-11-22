//! ZUT for parse_clang_tidy.rs: real clang-tidy 23.1.6 stdout (one_finding.txt, compile_error.txt; made in
//! /private/tmp/clang-tidy-fixtures.H87jA4 as fixtures/commands.txt says) and hand-written lines in its format, read
//! into hits. The expected positions are counted by hand from the source line.

use super::parse_clang_tidy;
use crate::types::{Hit, ParseError};
use procio::Output;
use rules::RelPath;
use std::path::Path;

const FIND: &str = "/private/tmp/clang-tidy-fixtures.H87jA4/find";
const BRACES: &str = "readability-braces-around-statements";

fn hit(path: &str, line: u32, col: u32, what: &str) -> Hit {
    Hit { path: RelPath::new(path), line: Some(line), col: Some(col), what: what.to_string() }
}

fn why(why: &str) -> Result<Vec<Hit>, ParseError> {
    Err(ParseError { why: why.to_string() })
}

#[test]
fn parse_clang_tidy_cases() {
    let one_finding: &[u8] = include_bytes!("fixtures/one_finding.txt");
    let braces = "statement should be inside braces";
    let rows: [(&str, i32, &[u8], &[u8], &str, &str, Result<Vec<Hit>, ParseError>); 10] = [
        ("the finding in a.c, none in the clean b.c", 0, one_finding, b"", BRACES, FIND, Ok(vec![hit("a.c", 1, 22, braces)])),
        ("a check not in the tag", 0, one_finding, b"", "readability-else-after-return", FIND, Ok(vec![])),
        ("a tag of two names, one the check", 0, b"/w/s/a.c:3:5: warning: two [misc-x,readability-braces-around-statements]\n", b"", BRACES, "/w", Ok(vec![hit("s/a.c", 3, 5, "two")])),
        ("an error line on exit 0", 0, b"/w/a.c:4:2: error: bad [readability-braces-around-statements]\n", b"", BRACES, "/w", Ok(vec![hit("a.c", 4, 2, "bad")])),
        ("a note line and snippet lines", 0, b"/w/a.c:2:1: note: see [readability-braces-around-statements]\n    2 | x: warning: y [readability-braces-around-statements]\n      | ^\n", b"", BRACES, "/w", Ok(vec![])),
        ("a path outside the folder", 0, b"/elsewhere/a.c:1:22: warning: w [readability-braces-around-statements]\n", b"", BRACES, "/w", Ok(vec![])),
        ("a relative path", 0, b"src/a.c:1:22: warning: w [readability-braces-around-statements]\n", b"", BRACES, "/w", why(
            "clang-tidy printed `src/a.c` relative to its compile database's directory; tm reads its findings only when the database's `file` entries are absolute paths",
        )),
        ("a relative path under another check", 0, b"src/a.c:1:22: warning: w [misc-x]\n", b"", BRACES, "/w", Ok(vec![])),
        ("exit 1 with a compile error quotes its error line", 1, include_bytes!("fixtures/compile_error.txt"), b"[1/2] Processing file /x/a.c.\n", BRACES, FIND, why(
            "clang-tidy gave no findings: /private/tmp/clang-tidy-fixtures.H87jA4/broken/a.c:1:27: error: use of undeclared identifier 'undefined_name' [clang-diagnostic-error]",
        )),
        ("exit 1 with no error line quotes stderr", 1, b"USAGE: clang-tidy [options] <source0>\n", b"Error: no checks enabled.\n", BRACES, FIND, why(
            "clang-tidy gave no findings: Error: no checks enabled.\n",
        )),
    ];
    for (label, code, stdout, stderr, check, folder, expected) in rows {
        let out = Output { code: Some(code), stdout: stdout.to_vec(), stderr: stderr.to_vec() };
        assert_eq!(parse_clang_tidy(&out, check, Path::new(folder), Path::new(folder)), expected, "{label}");
    }
}

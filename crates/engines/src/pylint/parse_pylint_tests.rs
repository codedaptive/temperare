//! ZUT for parse_pylint.rs: real pylint 4.1.4 json2 output (fixtures/, see fixtures.txt) read into hits. The
//! captured paths are relative to the folder pylint ran in, so `DIR` stands for it; no folder is read.

use super::parse_pylint;
use crate::types::{Hit, ParseError};
use procio::Output;
use rules::RelPath;
use std::path::Path;

const DIR: &str = "/work/project/py";

#[test]
fn parse_pylint_reads_the_checks_messages() {
    let c0121 = || Hit {
        path: RelPath::new("example.py"),
        line: Some(3),
        col: Some(4),
        what: "Comparison 'value == None' should be 'value is None'".to_string(),
    };
    let no_files = "pylint printed no JSON findings: No files to lint: exiting.\n".to_string();
    let rows: [(&str, i32, &[u8], &str, &str, Result<Vec<Hit>, ParseError>); 6] = [
        ("the C0121 fixture", 16, include_bytes!("fixtures/c0121.json"), "C0121", DIR, Ok(vec![c0121()])),
        ("a check matching nothing", 16, include_bytes!("fixtures/c0121.json"), "C0123", DIR, Ok(vec![])),
        ("the clean fixture", 0, include_bytes!("fixtures/clean.json"), "C0121", DIR, Ok(vec![])),
        ("a Command line W0012 is no hit", 0, include_bytes!("fixtures/unknown_id.json"), "W0012", DIR, Ok(vec![])),
        ("a file outside the folder", 16, include_bytes!("fixtures/c0121.json"), "C0121", "/work/project/py/sub", Ok(vec![])),
        ("the no-files text", 32, b"No files to lint: exiting.\n", "C0121", DIR, Err(ParseError { why: no_files })),
    ];
    for (name, code, stdout, check, folder, want) in rows {
        let out = Output { code: Some(code), stdout: stdout.to_vec(), stderr: Vec::new() };
        assert_eq!(parse_pylint(&out, check, Path::new(DIR), Path::new(folder)), want, "{name}");
    }
}

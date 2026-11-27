//! Unit test for parse_eslint.rs: real ESLint 10.15.0 output (fixtures/*.json; commands, files and exit codes in
//! fixtures/fixtures.txt) read into hits. Each capture keeps its folder under `/private/tmp/tm-eslint-fixtures`, which
//! the rows pass as `cwd` and `folder`; no folder is read. The not-JSON row is hand-built.

use super::parse_eslint;
use crate::types::{Hit, ParseError};
use procio::Output;
use rules::RelPath;
use std::path::Path;

const NV: &str = "/private/tmp/tm-eslint-fixtures/nv";
const SUP: &str = "/private/tmp/tm-eslint-fixtures/sup";
const FAT: &str = "/private/tmp/tm-eslint-fixtures/fat";
const NO_VAR: &str = "Unexpected var, use let or const instead.";

fn hit(path: &str, line: u32, col: u32, what: &str) -> Hit {
    Hit { path: RelPath::new(path), line: Some(line), col: Some(col), what: what.to_string() }
}

fn failed(why: &str) -> Result<Vec<Hit>, ParseError> {
    Err(ParseError { why: why.to_string() })
}

#[test]
fn parse_eslint_reads_the_checks_messages_or_fails() {
    let two_rules: &[u8] = include_bytes!("fixtures/no_var_and_unused.json");
    let fatal: &[u8] = include_bytes!("fixtures/fatal.json");
    let rows: [(&str, &[u8], &[u8], &str, &str, String, Result<Vec<Hit>, ParseError>); 8] = [
        ("the check picks one rule", two_rules, b"", "no-var", NV, NV.into(), Ok(vec![hit("example.js", 1, 1, NO_VAR)])),
        ("the other rule in order", two_rules, b"", "no-unused-vars", NV, NV.into(), Ok(vec![
            hit("example.js", 1, 5, "'x' is assigned a value but never used."),
            hit("example.js", 2, 5, "'y' is assigned a value but never used."),
        ])),
        ("a file outside the folder", two_rules, b"", "no-var", NV, format!("{NV}/sub"), Ok(vec![])),
        ("a bulk-suppressed finding", include_bytes!("fixtures/suppressed.json"), b"", "no-var", SUP, SUP.into(), Ok(vec![hit("example.js", 1, 1, NO_VAR)])),
        ("no file linted", include_bytes!("fixtures/no_files.json"), b"", "no-var", NV, NV.into(), Ok(vec![])),
        ("a fatal message", fatal, b"", "no-var", FAT, FAT.into(), failed("eslint could not parse broken.js: Parsing error: Unexpected token =")),
        ("a fatal message outside the folder", fatal, b"", "no-var", FAT, format!("{FAT}/sub"), failed("eslint could not parse /private/tmp/tm-eslint-fixtures/fat/broken.js: Parsing error: Unexpected token =")),
        ("not JSON", b"Oops", b"ESLint: 10.15.0 crashed", "no-var", NV, NV.into(), failed("eslint printed no JSON findings: ESLint: 10.15.0 crashed")),
    ];
    let failed: Vec<&str> = rows
        .iter()
        .filter(|(_, stdout, stderr, check, cwd, folder, want)| {
            let out = Output { code: Some(1), stdout: stdout.to_vec(), stderr: stderr.to_vec() };
            parse_eslint(&out, check, Path::new(cwd), Path::new(folder)) != *want
        })
        .map(|(name, ..)| *name)
        .collect();
    assert!(failed.is_empty(), "failed rows: {failed:?}");
}

//! Unit tests for parse_cargo_audit.rs: real cargo-audit 0.23.0 reports (fixtures/commands.txt) read for each kind of check,
//! the audit.toml guard and output that is not a report.

use super::parse_cargo_audit;
use crate::types::{Hit, ParseError};
use procio::Output;
use rules::RelPath;
use std::path::Path;

const VULNERABLE: &[u8] = include_bytes!("fixtures/vulnerable.json");
const UNMAINTAINED: &[u8] = include_bytes!("fixtures/unmaintained.json");
const CLEAN: &[u8] = include_bytes!("fixtures/clean.json");
const IGNORED: &[u8] = include_bytes!("fixtures/ignored.json");

#[test]
fn parse_cargo_audit_cases() {
    let hit = |what: &str| Ok(vec![Hit { path: RelPath::new("Cargo.lock"), line: None, col: None, what: what.to_string() }]);
    let smallvec = hit("RUSTSEC-2021-0003: smallvec 1.6.0: Buffer overflow in SmallVec::insert_many; upgrade to >=0.6.14, <1.0.0 or >=1.6.1");
    let ansi_term = hit("RUSTSEC-2021-0139: ansi_term 0.12.1: ansi_term is Unmaintained; no patched version");
    let why = |why: &str| Err(ParseError { why: why.to_string() });
    let rows: [(&str, i32, &[u8], &[u8], &str, Result<Vec<Hit>, ParseError>); 11] = [
        ("vulnerable by id", 1, VULNERABLE, b"", "RUSTSEC-2021-0003", smallvec.clone()),
        ("vulnerable by kind", 1, VULNERABLE, b"", "vulnerability", smallvec.clone()),
        ("vulnerable by all", 1, VULNERABLE, b"", "all", smallvec),
        ("vulnerable under another kind", 1, VULNERABLE, b"", "unmaintained", Ok(vec![])),
        ("unmaintained by kind", 0, UNMAINTAINED, b"", "unmaintained", ansi_term.clone()),
        ("unmaintained by id", 0, UNMAINTAINED, b"", "RUSTSEC-2021-0139", ansi_term),
        ("a kind that matches nothing", 0, UNMAINTAINED, b"", "unsound", Ok(vec![])),
        ("clean", 0, CLEAN, b"", "all", Ok(vec![])),
        ("an audit.toml ignoring an advisory", 0, IGNORED, b"", "all", why(
            "cargo-audit applied an audit.toml cctao was not given (ignore = [\"RUSTSEC-2021-0003\"]); the rule file is the only home for an exception, so cctao reads no findings under it",
        )),
        ("no database", 1, b"", b"error: error loading advisory database: I/O operation failed\n", "all", why(
            "cargo-audit printed no JSON report: error: error loading advisory database: I/O operation failed\n",
        )),
        ("text on stdout", 0, b"Fetching advisory database\n", b"", "all", why("cargo-audit printed no JSON report: Fetching advisory database\n")),
    ];
    for (label, code, stdout, stderr, check, expected) in rows {
        let out = Output { code: Some(code), stdout: stdout.to_vec(), stderr: stderr.to_vec() };
        assert_eq!(parse_cargo_audit(&out, check, Path::new("/w"), Path::new("/w")), expected, "{label}");
    }
}

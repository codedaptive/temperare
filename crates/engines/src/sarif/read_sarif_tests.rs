//! ZUT for read_sarif.rs: its two failures kept apart. parse_sarif_tests.rs carries the hits themselves.

use super::read_sarif;
use super::sarif_failure::SarifFailure;
use crate::types::{Hit, ParseError};
use procio::Output;
use std::path::Path;

const DIR: &str = "/work/project";
const OUTSIDE_LOG: &[u8] = br#"{"runs":[{"results":[
    {"ruleId":"R1","locations":[{"physicalLocation":{"artifactLocation":{"uri":"file:///elsewhere/d.py"}}}]}
]}]}"#;

#[test]
fn read_sarif_tells_not_a_log_from_unplaced() {
    let rows: [(&str, &[u8], &str, Result<Vec<Hit>, SarifFailure>); 3] = [
        ("stdout that is not JSON", b"no log here\n", "R1", Err(SarifFailure::NotALog(ParseError {
            why: "SARIF output is not a SARIF log: no log here\n".to_string(),
        }))),
        ("a checked result outside the folder", OUTSIDE_LOG, "R1", Err(SarifFailure::Unplaced(ParseError {
            why: "SARIF result names `file:///elsewhere/d.py`, a path not under the rule folder `/work/project`".to_string(),
        }))),
        ("an outside result of a rule not checked", OUTSIDE_LOG, "R2", Ok(vec![])),
    ];
    for (name, stdout, check, want) in rows {
        let out = Output { code: Some(0), stdout: stdout.to_vec(), stderr: Vec::new() };
        assert_eq!(read_sarif(&out, check, Path::new(DIR), Path::new(DIR)), want, "{name}");
    }
}

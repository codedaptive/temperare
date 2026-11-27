//! Unit test for parse_pmd.rs: PMD 7.29.0's real SARIF logs (fixtures/, see fixtures.txt) read by exit code. `DIR` is the
//! folder the violation was captured in; no folder is read.

use super::parse_pmd;
use crate::types::{Hit, ParseError};
use procio::Output;
use rules::RelPath;
use std::path::Path;

const DIR: &str = "/private/tmp/tm-pmd-capture/data/viol";
const CHECK: &str = "category/java/bestpractices.xml/UnusedPrivateField";

#[test]
fn parse_pmd_reads_the_log_on_exit_0_or_4_and_fails_otherwise() {
    let hit = Hit {
        path: RelPath::new("Foo.java"),
        line: Some(2),
        col: Some(17),
        what: "Avoid unused private fields such as 'unused'.".to_string(),
    };
    let unchecked = ParseError { why: "pmd could not process every file (exit 5), so files went unchecked".to_string() };
    let ruleset = "[ERROR] Cannot load ruleset category/java/nosuch.xml/Foo: Cannot resolve rule/ruleset reference 'category/java/nosuch.xml/Foo'.  Make sure the resource is a valid file or URL and is on the CLASSPATH. ";
    let rows: [(&str, Option<i32>, &[u8], &[u8], Result<Vec<Hit>, ParseError>); 5] = [
        ("a violation", Some(4), include_bytes!("fixtures/violation.sarif"), b"[INFO] Found 1 violation.\n", Ok(vec![hit])),
        ("clean", Some(0), include_bytes!("fixtures/clean.sarif"), b"", Ok(vec![])),
        ("a file pmd could not parse", Some(5), include_bytes!("fixtures/parse_error.sarif"), include_bytes!("fixtures/parse_error.stderr"), Err(unchecked)),
        ("a ruleset pmd could not load", Some(1), b"", include_bytes!("fixtures/unknown_category.stderr"), Err(ParseError { why: format!("pmd exited Some(1): {ruleset}") })),
        ("killed", None, b"", b"", Err(ParseError { why: "pmd exited None: ".to_string() })),
    ];
    for (name, code, stdout, stderr, want) in rows {
        let out = Output { code, stdout: stdout.to_vec(), stderr: stderr.to_vec() };
        assert_eq!(parse_pmd(&out, CHECK, Path::new(DIR), Path::new(DIR)), want, "{name}");
    }
}

//! Unit test for parse_phpstan.rs on real PHPStan 2.3.7 output (fixtures/fixtures.txt records each capture). The captures
//! keep their capture folder's absolute paths, so `DIR` is passed as `cwd`, and as `folder` in every row but the
//! folder-holding-none row; no folder is read.

use super::parse_phpstan;
use crate::types::{Hit, ParseError};
use procio::Output;
use rules::RelPath;
use std::path::Path;

const DIR: &str = "/private/tmp/tm-phpstan-capture";

fn hit(path: &str, line: Option<u32>, what: &str) -> Hit {
    Hit { path: RelPath::new(path), line, col: None, what: what.to_string() }
}

#[test]
fn parse_phpstan_cases() {
    let findings = vec![
        hit("src/example.php", Some(3), "Parameter #1 $value of function takeInt expects int, string given."),
        hit("src/example.php", Some(7), "Function half() should return string but returns int."),
        hit("src/example.php", Some(10), "Function undefinedFunction not found."),
        hit("src/uses_trait.php", Some(6), "Call to an undefined method UsesT::missing()."),
    ];
    let crashed = "phpstan reported an error outside any file: Child process error: PHPStan process crashed because it \
        reached configured PHP memory limit: 72M\nIncrease your memory limit in php.ini or run PHPStan with \
        --memory-limit CLI option.\n while running parallel worker";
    // Built by hand: a message PHPStan ties to no line, and a file key in another folder.
    let null_line = br#"{"totals":{"errors":0,"file_errors":2},"files":{"/elsewhere/a.php":{"errors":1,"messages":[{"message":"Outside.","line":1}]},"/private/tmp/tm-phpstan-capture/b.php":{"errors":1,"messages":[{"message":"No line.","line":null}]}},"errors":[]}"#;
    let rows: [(&str, &[u8], &[u8], &str, Result<Vec<Hit>, ParseError>); 7] = [
        ("findings, trait key cut", include_bytes!("fixtures/findings.json"), b"", DIR, Ok(findings)),
        ("findings, folder holding none", include_bytes!("fixtures/findings.json"), b"", "/private/tmp/tm-phpstan-capture/lib", Ok(vec![])),
        ("clean", include_bytes!("fixtures/clean.json"), b"", DIR, Ok(vec![])),
        ("crashed worker", include_bytes!("fixtures/crashed.json"), b"", DIR, Err(ParseError { why: crashed.to_string() })),
        ("null line, file outside", null_line, b"", DIR, Ok(vec![hit("b.php", None, "No line.")])),
        ("no stdout, stderr quoted", b"", include_bytes!("fixtures/no_files.txt"), DIR,
            Err(ParseError { why: format!("phpstan printed no JSON report: \n [ERROR] No files found to analyse.{}\n\n", " ".repeat(45)) })),
        ("not the report, stdout quoted", b"[]", b"", DIR, Err(ParseError { why: "phpstan printed no JSON report: []".to_string() })),
    ];
    for (label, stdout, stderr, folder, expected) in rows {
        let out = Output { code: Some(1), stdout: stdout.to_vec(), stderr: stderr.to_vec() };
        assert_eq!(parse_phpstan(&out, "5", Path::new(DIR), Path::new(folder)), expected, "{label}");
    }
}

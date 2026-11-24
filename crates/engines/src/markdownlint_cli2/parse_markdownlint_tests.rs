//! ZUT for parse_markdownlint.rs on real markdownlint-cli2 0.24.2 output (fixtures/fixtures.txt records each capture).
//! Its file names are relative to the capture folder, passed as `cwd`, and as `folder` except where a row names
//! another; no folder is read.

use super::parse_markdownlint;
use crate::types::{Hit, ParseError};
use procio::Output;
use rules::RelPath;
use std::path::Path;

const DIR: &str = "/private/tmp/tm-mdl-capture";

fn hit(path: &str, line: u32, col: Option<u32>, what: &str) -> Hit {
    Hit { path: RelPath::new(path), line: Some(line), col, what: what.to_string() }
}

#[test]
fn parse_markdownlint_cases() {
    let findings: &[u8] = include_bytes!("fixtures/findings.json");
    let notes: &[u8] = include_bytes!("fixtures/notes.json");
    let md041 = "MD041/first-line-heading/first-line-h1 First line in a file should be a top-level heading [Context: \"Not a heading.\"]";
    // Built by hand: an empty detail and context, which the default formatter leaves out.
    let empty = br#"[{"fileName":"a.md","lineNumber":2,"ruleNames":["MD013","line-length"],"ruleDescription":"Line length","ruleInformation":null,"errorDetail":"","errorContext":"","errorRange":[3,1],"fixInfo":null,"severity":"error"}]"#;
    let banner = b"markdownlint-cli2 v0.24.2 (markdownlint v0.42.2)\n";
    let rows: [(&str, &[u8], &str, &str, Result<Vec<Hit>, ParseError>); 12] = [
        ("id, with detail", findings, "MD013", DIR,
            Ok(vec![hit("findings.md", 4, Some(81), "MD013/line-length Line length [Expected: 80; Actual: 105]")])),
        ("alias", findings, "no-trailing-spaces", DIR,
            Ok(vec![hit("findings.md", 5, Some(22), "MD009/no-trailing-spaces Trailing spaces [Expected: 0 or 2; Actual: 3]")])),
        ("lower case, with context", findings, "md018", DIR,
            Ok(vec![hit("findings.md", 7, Some(1), "MD018/no-missing-space-atx No space after hash on atx style heading [Context: \"#Missing space\"]")])),
        ("a rule not reported", findings, "MD041", DIR, Ok(vec![])),
        ("no range", notes, "first-line-h1", DIR, Ok(vec![hit("sub/notes.markdown", 1, None, md041)])),
        ("folder below the run's", notes, "MD041", "/private/tmp/tm-mdl-capture/sub", Ok(vec![hit("notes.markdown", 1, None, md041)])),
        ("folder holding none", notes, "MD041", "/private/tmp/tm-mdl-capture/other", Ok(vec![])),
        ("clean", include_bytes!("fixtures/clean.json"), "MD013", DIR, Ok(vec![])),
        ("empty detail and context", empty, "MD013", DIR, Ok(vec![hit("a.md", 2, Some(3), "MD013/line-length Line length")])),
        ("banner, no array", banner, "MD013", DIR,
            Err(ParseError { why: "markdownlint-cli2 printed no JSON results: markdownlint-cli2 v0.24.2 (markdownlint v0.42.2)\n".to_string() })),
        ("empty stdout", b"", "MD013", DIR, Err(ParseError { why: "markdownlint-cli2 printed no JSON results: ".to_string() })),
        ("not an array", b"{}", "MD013", DIR, Err(ParseError { why: "markdownlint-cli2 printed no JSON results: {}".to_string() })),
    ];
    for (label, stdout, check, folder, expected) in rows {
        let out = Output { code: Some(1), stdout: stdout.to_vec(), stderr: Vec::new() };
        assert_eq!(parse_markdownlint(&out, check, Path::new(DIR), Path::new(folder)), expected, "{label}");
    }
}

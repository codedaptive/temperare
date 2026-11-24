//! ZUT for parse_vale.rs: real Vale 3.28.1 output, captured once (fixtures/fixtures.txt says how), read into hits.
//! The captures keep the capture folder's paths relative, so the tests pass it as `cwd`; `folder` is it or its `sub`.
//! The W101-only stderr and the non-JSON stdout rows are hand-built in the forms Vale prints.

use super::parse_vale;
use crate::types::{Hit, ParseError};
use procio::Output;
use rules::RelPath;
use std::path::Path;

const DIR: &str = "/private/tmp/tm-vale-capture";

fn hit(path: &str, line: u32, col: u32, what: &str) -> Hit {
    Hit { path: RelPath::new(path), line: Some(line), col: Some(col), what: what.to_string() }
}

#[test]
fn parse_vale_cases() {
    let findings: &[u8] = include_bytes!("fixtures/findings.json");
    let w101: &[u8] = b"W101 grammar \"c\" is not installed; add 'Grammars = c' to .vale.ini and run 'vale sync'; until then, comments are read by their delimiters\n";
    let sentense = "Did you really mean 'sentense'?";
    let err = |why: &str| Err(ParseError { why: why.to_string() });
    let rows: [(&str, &[u8], &[u8], &str, &str, Result<Vec<Hit>, ParseError>); 9] = [
        (
            "the check's alerts, by path",
            findings,
            b"",
            "Vale.Spelling",
            "",
            Ok(vec![
                hit("doc.md", 1, 3, sentense),
                hit("doc.md", 3, 12, "Did you really mean 'mispeled'?"),
                hit("sub/other.md", 1, 9, sentense),
            ]),
        ),
        ("another rule's alerts", findings, b"", "Vale.Repetition", "", Ok(vec![hit("doc.md", 3, 6, "'is' is repeated!")])),
        ("outside the folder dropped", findings, b"", "Vale.Spelling", "sub", Ok(vec![hit("other.md", 1, 9, sentense)])),
        ("only an exact name", findings, b"", "Vale.Spellin", "", Ok(vec![])),
        ("clean", include_bytes!("fixtures/clean.json"), b"", "Vale.Spelling", "", Ok(vec![])),
        ("W101 is no error", findings, w101, "Vale.Repetition", "", Ok(vec![hit("doc.md", 3, 6, "'is' is repeated!")])),
        (
            "an error beside findings",
            include_bytes!("fixtures/dita.json"),
            include_bytes!("fixtures/dita.txt"),
            "Vale.Spelling",
            "",
            err("vale stopped with E100: dita not found; it's needed to read DITA"),
        ),
        (
            "an error alone",
            b"",
            include_bytes!("fixtures/missing_style.txt"),
            "Gone.Rule",
            "",
            err("vale stopped with E201: Style 'Gone' isn't on the StylesPath (/private/tmp/tm-vale-capture/gone/styles)."),
        ),
        ("not JSON", b"garbage", b"", "Vale.Spelling", "", err("vale printed no JSON findings: garbage")),
    ];
    for (label, stdout, stderr, check, sub, expected) in rows {
        let out = Output { code: Some(1), stdout: stdout.to_vec(), stderr: stderr.to_vec() };
        let folder = Path::new(DIR).join(sub);
        assert_eq!(parse_vale(&out, check, Path::new(DIR), &folder), expected, "{label}");
    }
}

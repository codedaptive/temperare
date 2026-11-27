//! Unit test for parse_stylelint.rs: real stylelint 18.1.0 stderr (fixtures/*.txt; commands, files and exit codes in
//! fixtures/fixtures.txt) read into hits. Each capture keeps its folder under `/private/tmp/tm-stylelint-fixtures`,
//! which the rows pass as `cwd` and `folder`; no folder is read.

use super::parse_stylelint;
use crate::types::{Hit, ParseError};
use procio::Output;
use rules::RelPath;
use std::path::Path;

const ROOT: &str = "/private/tmp/tm-stylelint-fixtures";
const HEX: &str = "color-no-invalid-hex";

fn hit(path: &str, line: u32, col: u32, what: &str) -> Hit {
    Hit { path: RelPath::new(path), line: Some(line), col: Some(col), what: what.to_string() }
}

fn failed(why: &str) -> Result<Vec<Hit>, ParseError> {
    Err(ParseError { why: why.to_string() })
}

#[test]
fn parse_stylelint_reads_the_checks_warnings_or_fails() {
    let two: &[u8] = include_bytes!("fixtures/findings.txt");
    let a_hex = || hit("a.css", 1, 12, "Invalid hex color \"#fff1az\" (color-no-invalid-hex)");
    let rows: [(&str, &[u8], &str, &str, &str, Result<Vec<Hit>, ParseError>); 8] = [
        ("the check over two files", two, HEX, "two", "two", Ok(vec![
            a_hex(),
            hit("b.css", 2, 12, "Invalid hex color \"#12345g\" (color-no-invalid-hex)"),
        ])),
        ("the other rule", two, "color-hex-length", "two", "two", Ok(vec![
            hit("b.css", 1, 12, "Expected \"#ffffff\" to be \"#fff\" (color-hex-length)"),
        ])),
        ("files outside the folder", two, HEX, "two", "two/sub", Ok(vec![])),
        ("clean", include_bytes!("fixtures/clean.txt"), HEX, "clean", "clean", Ok(vec![])),
        ("another rule unknown", include_bytes!("fixtures/unknown.txt"), HEX, "unk", "unk", Ok(vec![a_hex()])),
        ("ignored", include_bytes!("fixtures/ignored.txt"), HEX, "ign", "ign", failed("stylelint ignored a.css through ignoreFiles in its config; exceptions go in the rule file")),
        ("a CSS syntax error", include_bytes!("fixtures/syntax_error.txt"), HEX, "syn", "syn", failed("stylelint could not parse broken.css: Unclosed block (CssSyntaxError)")),
        ("no report", include_bytes!("fixtures/config_error.txt"), HEX, "cfgerr", "cfgerr", failed("stylelint printed no JSON report: ConfigurationError: Could not find \"nope-config\". Do you need to install the package, use the \"configBasedir\" option or add the filename and extension to the path?\n    at getModulePath (file:///opt/ho")),
    ];
    let failed: Vec<&str> = rows
        .iter()
        .filter(|(_, stderr, check, cwd, folder, want)| {
            let out = Output { code: Some(2), stdout: Vec::new(), stderr: stderr.to_vec() };
            let (cwd, folder) = (format!("{ROOT}/{cwd}"), format!("{ROOT}/{folder}"));
            parse_stylelint(&out, check, Path::new(&cwd), Path::new(&folder)) != *want
        })
        .map(|(name, ..)| *name)
        .collect();
    assert!(failed.is_empty(), "failed rows: {failed:?}");
}

//! Unit test for parse_detekt.rs: real detekt 1.23.8 SARIF reports (fixtures/, see fixtures/commands.txt) read into hits,
//! and outputs for the exits that are no answer (exit 1's is detekt's real first line, recorded in commands.txt; exit
//! 3's and no exit's are hand-made). The reports keep their scratch folder's absolute URIs, so
//! each row passes that folder as `cwd` and `folder`; nothing is read from it.

use super::parse_detekt;
use crate::types::{Hit, ParseError};
use procio::Output;
use rules::RelPath;
use std::path::Path;

const MAGIC: &str = "/private/tmp/tm-detekt-fix/magic";
const CLEAN: &str = "/private/tmp/tm-detekt-fix/clean";
const MESSAGE: &str = "This expression contains a magic number. Consider defining it to a well named constant.";
const LICENSE_CRASH: &str = "java.lang.IllegalArgumentException: Rule 'AbsentOrWrongFileLicense': License template file \
                             not found at `/private/tmp/tm-detekt-kong/lic/license.template`.\n";

fn output(code: Option<i32>, stdout: &[u8], stderr: &[u8]) -> Output {
    Output { code, stdout: stdout.to_vec(), stderr: stderr.to_vec() }
}

#[test]
fn parse_detekt_reads_the_picked_rules_hits() {
    let magic = || output(Some(2), include_bytes!("fixtures/magic_number.sarif"), b"Analysis failed with 1 weighted issues.\n");
    let demo = || Ok(vec![Hit { path: RelPath::new("src/Demo.kt"), line: Some(4), col: Some(20), what: MESSAGE.to_string() }]);
    let no_answer = |why: &str| Err(ParseError { why: why.to_string() });
    let rows: Vec<(&str, Output, &str, &str, Result<Vec<Hit>, ParseError>)> = vec![
        ("the rule's finding", magic(), "MagicNumber", MAGIC, demo()),
        ("the rule's set picks it", magic(), "style", MAGIC, demo()),
        ("another rule of the set has none", magic(), "UnusedPrivateMember", MAGIC, Ok(vec![])),
        ("a clean report", output(Some(0), include_bytes!("fixtures/clean.sarif"), b"1 kotlin file was analyzed.\n"), "MagicNumber", CLEAN, Ok(vec![])),
        (
            "exit 3, an invalid config",
            output(Some(3), b"", b"Property 'style>NoSuchRuleXyz' is misspelled or does not exist.\n"),
            "MagicNumber",
            MAGIC,
            no_answer("detekt gave no answer (exit 3): Property 'style>NoSuchRuleXyz' is misspelled or does not exist.\n"),
        ),
        // The first line of detekt's real stderr for AbsentOrWrongFileLicense with no license template
        // (fixtures/commands.txt), the stack trace after it left out.
        (
            "exit 1, a crash",
            output(Some(1), b"", LICENSE_CRASH.as_bytes()),
            "AbsentOrWrongFileLicense",
            MAGIC,
            no_answer(&format!("detekt gave no answer (exit 1): {LICENSE_CRASH}")),
        ),
        ("no exit code", output(None, b"", b"Killed\n"), "MagicNumber", MAGIC, no_answer("detekt gave no answer (exit none): Killed\n")),
    ];
    let wrong: Vec<String> = rows
        .into_iter()
        .filter_map(|(name, out, check, folder, want)| {
            let got = parse_detekt(&out, check, Path::new(folder), Path::new(folder));
            (got != want).then(|| format!("{name}: got {got:?}, want {want:?}"))
        })
        .collect();
    assert!(wrong.is_empty(), "{wrong:#?}");
}

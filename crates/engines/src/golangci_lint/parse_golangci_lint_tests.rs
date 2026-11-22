//! ZUT for parse_golangci_lint.rs: output read into hits. The fixture rows are real golangci-lint 2.15.0 output
//! (fixtures/*, described in fixtures/how_made.txt); the text-after-JSON and null-Issues rows are hand-made shapes the
//! reader must handle. The captures keep their scratch folders' absolute paths, so each row passes that folder as
//! `cwd` (and as `folder`, except the outside-the-folder row); nothing is read from it.

use super::parse_golangci_lint;
use crate::types::{Hit, ParseError};
use procio::Output;
use rules::RelPath;
use std::path::Path;

const ISSUES: &str = "/private/tmp/gcl-fix/issues";
const CLEAN: &str = "/private/tmp/gcl-fix/clean";
const TYPECHECK: &str = "/private/tmp/gcl-fix/typecheck";
const NO_MODULE: &str = "/private/tmp/gcl-fix/nomod";

const SA1019: &str = "SA1019: strings.Title has been deprecated since Go 1.18 and an alternative has been available \
                      since Go 1.0: The rule Title uses for word boundaries does not handle Unicode punctuation \
                      properly. Use golang.org/x/text/cases instead.";
const SA4000: &str = "SA4000: identical expressions on the left and right side of the '==' operator";
const COPYLOCKS: &str = "copylocks: Locked passes lock by value: sync.Mutex";

fn output(code: i32, stdout: &[u8], stderr: &[u8]) -> Output {
    Output { code: Some(code), stdout: stdout.to_vec(), stderr: stderr.to_vec() }
}

fn hit(line: u32, col: u32, what: &str) -> Hit {
    Hit { path: RelPath::new("probe.go"), line: Some(line), col: Some(col), what: what.to_string() }
}

fn failed(why: &str) -> Result<Vec<Hit>, ParseError> {
    Err(ParseError { why: why.to_string() })
}

#[test]
fn parse_golangci_lint_reads_the_checked_linters_issues() {
    let issues = || output(1, include_bytes!("fixtures/staticcheck_and_govet.json"), b"");
    let rows: Vec<(&str, Output, &str, &str, &str, Result<Vec<Hit>, ParseError>)> = vec![
        (
            "staticcheck keeps its two issues, trimmed, and drops govet's",
            issues(),
            "staticcheck",
            ISSUES,
            ISSUES,
            Ok(vec![hit(9, 9, SA1019), hit(13, 9, SA4000)]),
        ),
        (
            "a comma check keeps both linters' issues in output order",
            issues(),
            "staticcheck,govet",
            ISSUES,
            ISSUES,
            Ok(vec![hit(16, 16, COPYLOCKS), hit(9, 9, SA1019), hit(13, 9, SA4000)]),
        ),
        ("an issue in a file outside the folder is not a hit", issues(), "staticcheck", ISSUES, "/private/tmp/gcl-fix/issues/sub", Ok(vec![])),
        ("a clean run has no hits", output(0, include_bytes!("fixtures/clean.json"), b""), "staticcheck", CLEAN, CLEAN, Ok(vec![])),
        (
            "a typecheck issue fails the run, quoting its text",
            output(1, include_bytes!("fixtures/typecheck.json"), b""),
            "staticcheck",
            TYPECHECK,
            TYPECHECK,
            failed(
                "golangci-lint could not load the code, so its linters did not check it: could not import \
                 example.invalid/none (uses.go:3:8: missing go.sum entry for module providing package \
                 example.invalid/none (imported by example.com/typecheck); to add:\n\tgo get example.com/typecheck)",
            ),
        ),
        (
            "exit 7 with no go.mod is no answer, never clean",
            output(7, include_bytes!("fixtures/no_module.json"), include_bytes!("fixtures/no_module.stderr")),
            "staticcheck",
            NO_MODULE,
            NO_MODULE,
            failed(
                "golangci-lint gave no answer (exit 7): level=error msg=\"[linters_context] typechecking error: \
                 pattern ./...: directory prefix . does not contain main module or its selected dependencies\"\n",
            ),
        ),
        (
            "stdout with text after the JSON is not the JSON",
            output(0, b"{\"Issues\":[]}\n0 issues.\n", b""),
            "staticcheck",
            CLEAN,
            CLEAN,
            failed("golangci-lint printed no JSON findings: {\"Issues\":[]}\n0 issues.\n"),
        ),
        ("null Issues reads as none", output(0, b"{\"Issues\":null}", b""), "staticcheck", CLEAN, CLEAN, Ok(vec![])),
    ];
    let wrong: Vec<String> = rows
        .into_iter()
        .filter_map(|(name, out, check, cwd, folder, want)| {
            let got = parse_golangci_lint(&out, check, Path::new(cwd), Path::new(folder));
            (got != want).then(|| format!("{name}: got {got:?}, want {want:?}"))
        })
        .collect();
    assert!(wrong.is_empty(), "{wrong:#?}");
}

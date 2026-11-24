//! ZUT for run_eslint_with.rs: the flow on a fake host answering with real ESLint 10.15.0 output (fixtures/; see
//! fixtures/fixtures.txt) and a fake listing: the `--rule` pair the run is given, hits kept or dropped by the listing,
//! the no-file case (by the listing and by the rule's paths), a fatal message, an unknown check, and a param, which is refused before any run. Nothing runs; no folder
//! is read. `EMPTY_CONFIG_WARNING` is the stderr the fatal capture printed, as fixtures.txt records it.

use super::{run_eslint_with, ESLINT};
use crate::host::Host;
use crate::types::{Hit, Stop};
use procio::{Cmd, Output};
use rules::{RelPath, RuleFile};
use std::cell::{Cell, RefCell};
use std::ffi::OsString;
use std::path::Path;

const NV: &str = "/private/tmp/tm-eslint-fixtures/nv";
const FAT: &str = "/private/tmp/tm-eslint-fixtures/fat";
const EMPTY_CONFIG_WARNING: &str = "(node:898) ESLintEmptyConfigWarning: Running ESLint with an empty config (from /private/tmp/tm-eslint-fixtures/fat/eslint.config.mjs). Please double-check that this is what you want. If you want to run ESLint with an empty config, export [{}] to remove this warning.\n(Use `node --trace-warnings ...` to show where the warning was created)\n";
const NO_FILE: &str = "ESLint linted no file in the rule's paths that git lists (its config decides which files it lints)";

struct Row {
    name: &'static str,
    check: &'static str,
    params: &'static str,
    folder: &'static str,
    answer: (i32, &'static [u8], &'static [u8]),
    listing: Option<&'static [&'static str]>,
    want: Result<Vec<Hit>, Stop>,
    /// The args of the one run asked for, or None when nothing may run.
    args: Option<&'static [&'static str]>,
    counted: bool,
}

fn no_var_hit() -> Hit {
    Hit { path: RelPath::new("example.js"), line: Some(1), col: Some(1), what: "Unexpected var, use let or const instead.".into() }
}

const FIXED: [&str; 6] =
    ["--format", "json", "--no-inline-config", "--no-ignore", "--pass-on-unpruned-suppressions", "--no-error-on-unmatched-pattern"];
const NO_VAR_ARGS: &[&str] = &[FIXED[0], FIXED[1], FIXED[2], FIXED[3], FIXED[4], FIXED[5], "--rule", "{\"no-var\":\"error\"}", "."];

/// The rule's result, the args of each run the host was asked for, and the count the host holds after.
fn run_row(row: &Row) -> (Result<Vec<Hit>, Stop>, Vec<Vec<OsString>>, Option<usize>) {
    let text = format!(
        "version = 1\n[[rule]]\nid = \"e\"\nengine = \"eslint\"\ncheck = \"{}\"\nfix = \"f\"\n{}",
        row.check, row.params
    );
    let folder = Path::new(row.folder);
    let file = RuleFile::parse(&text, folder).unwrap();
    let seen = RefCell::new(Vec::new());
    let (code, stdout, stderr) = row.answer;
    let run = |cmd: &Cmd| {
        seen.borrow_mut().push(cmd.args.clone());
        Ok(Output { code: Some(code), stdout: stdout.to_vec(), stderr: stderr.to_vec() })
    };
    let on_path = |_: &str| -> bool { panic!("run_eslint_with looks nothing up on PATH") };
    let raw_chars = Cell::new(None);
    let host = Host { run: &run, on_path: &on_path, raw_chars: &raw_chars, base: None };
    let listed = |at: &Path| {
        assert_eq!(at, folder);
        row.listing.map(|files| files.iter().map(|file| RelPath::new(file)).collect())
    };
    let got = run_eslint_with(&listed, &ESLINT, &file.rules[0], folder, &host);
    (got, seen.into_inner(), raw_chars.get())
}

#[test]
fn run_eslint_with_settles_hits_listing_and_stops() {
    let two_rules: &[u8] = include_bytes!("fixtures/no_var_and_unused.json");
    let no_file = || Err(Stop::NoFiles { why: NO_FILE.into() });
    let rows = [
        Row { name: "the --rule pair before .", check: "no-var", params: "", folder: NV, answer: (1, two_rules, b""), listing: None, want: Ok(vec![no_var_hit()]), args: Some(NO_VAR_ARGS), counted: true },
        Row { name: "listed hit kept", check: "no-var", params: "", folder: NV, answer: (1, two_rules, b""), listing: Some(&["example.js"]), want: Ok(vec![no_var_hit()]), args: Some(NO_VAR_ARGS), counted: true },
        Row { name: "unlisted hit dropped", check: "no-var", params: "", folder: NV, answer: (1, two_rules, b""), listing: Some(&["eslint.config.mjs"]), want: Ok(vec![]), args: Some(NO_VAR_ARGS), counted: true },
        Row { name: "nothing linted", check: "no-var", params: "", folder: NV, answer: (0, include_bytes!("fixtures/no_files.json"), b""), listing: None, want: no_file(), args: Some(NO_VAR_ARGS), counted: true },
        Row { name: "nothing listed linted", check: "no-var", params: "", folder: NV, answer: (1, two_rules, b""), listing: Some(&["notes.txt"]), want: no_file(), args: Some(NO_VAR_ARGS), counted: true },
        Row { name: "linted files outside the rule's paths", check: "no-var", params: "paths = [\"src/**\"]\n", folder: NV, answer: (1, two_rules, b""),
              listing: Some(&["eslint.config.mjs", "example.js"]), want: no_file(), args: Some(NO_VAR_ARGS), counted: true },
        Row { name: "fatal", check: "no-var", params: "", folder: FAT, answer: (1, include_bytes!("fixtures/fatal.json"), EMPTY_CONFIG_WARNING.as_bytes()), listing: None,
              want: Err(Stop::Failed { code: Some(1), stderr: "eslint could not parse broken.js: Parsing error: Unexpected token =".into() }), args: Some(NO_VAR_ARGS), counted: true },
        Row { name: "unknown check", check: "no-such-rule-xyz", params: "", folder: NV, answer: (2, b"", include_bytes!("fixtures/unknown_core.txt")), listing: None,
              want: Err(Stop::UnknownCheck { said: "TypeError: Key \"rules\": Key \"no-such-rule-xyz\": Could not find \"no-such-rule-xyz\" in plugin \"@\".".into() }),
              args: Some(&[FIXED[0], FIXED[1], FIXED[2], FIXED[3], FIXED[4], FIXED[5], "--rule", "{\"no-such-rule-xyz\":\"error\"}", "."]), counted: true },
        Row { name: "a param", check: "no-var", params: "params = { option = 1 }\n", folder: NV, answer: (1, two_rules, b""), listing: None,
              want: Err(Stop::Failed { code: None, stderr: "eslint param option: an eslint rule takes no params".into() }), args: None, counted: false },
    ];
    let mut failed = Vec::new();
    for row in &rows {
        let (got, runs, raw_chars) = run_row(row);
        let want_runs: Vec<Vec<OsString>> = row.args.iter().map(|args| args.iter().map(OsString::from).collect()).collect();
        if got != row.want || runs != want_runs || raw_chars.is_some() != row.counted {
            failed.push((row.name, got, runs, raw_chars));
        }
    }
    assert!(failed.is_empty(), "failed rows: {failed:#?}");
}

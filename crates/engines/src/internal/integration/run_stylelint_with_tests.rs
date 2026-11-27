//! ZIT for run_stylelint_with.rs: the flow on a fake host answering with real stylelint 18.1.0 output
//! (stylelint/fixtures/; see stylelint/fixtures/fixtures.txt) and a fake listing: no file, the check off, a config turning on fix, findings with the exact
//! probe and check-run arguments, an ignored file, an unknown check, and a param. The fake host records every command's
//! arguments, with the suppressions file's path replaced by `<empty file>` after reading what it holds at the run; the
//! file must be gone once the flow returns. No engine runs; the host is fake. The one real file is the flow's own
//! temporary suppressions file under the system temp folder, read and checked removed.

use crate::stylelint::{run_stylelint_with, STYLELINT};
use crate::host::Host;
use crate::types::{Hit, Stop};
use procio::{Cmd, Output};
use rules::{RelPath, RuleFile};
use std::cell::{Cell, RefCell};
use std::path::{Path, PathBuf};

const ROOT: &str = "/private/tmp/tm-stylelint-fixtures";
const RULES: &[u8] = include_bytes!("../../stylelint/fixtures/print_config_rules.json");
const FIX: &[u8] = include_bytes!("../../stylelint/fixtures/print_config_fix.json");
const SUP: &str = "--suppress-location=<empty file>";
const FIXED: [&str; 7] = [
    "--formatter", "json", "--ignore-disables", "--ignore-path=/dev/null", "--disable-default-ignores", "--allow-empty-input", "--no-cache",
];

struct Row {
    name: &'static str,
    check: &'static str,
    /// Rule-file lines after `fix` (paths, config, params).
    extra: &'static str,
    folder: &'static str,
    listed: &'static [&'static str],
    /// The stdout of every `--print-config` probe (exit 0).
    probe: &'static [u8],
    /// The check run's exit code and stderr.
    answer: (i32, &'static [u8]),
    want: Result<Vec<Hit>, Stop>,
    runs: Vec<Vec<&'static str>>,
    counted: bool,
}

fn hit(path: &str, line: u32, what: &str) -> Hit {
    Hit { path: RelPath::new(path), line: Some(line), col: Some(12), what: what.to_string() }
}

fn probe(file: &'static str) -> Vec<&'static str> {
    vec!["--print-config", file]
}

fn check_run(tail: &[&'static str]) -> Vec<&'static str> {
    FIXED.iter().copied().chain(tail.iter().copied()).collect()
}

/// The rule's result, each command's args, the count the host holds after, and the suppressions files the check runs
/// were given, each with what it held at the run.
fn run_row(row: &Row) -> (Result<Vec<Hit>, Stop>, Vec<Vec<String>>, Option<usize>, Vec<(PathBuf, Option<String>)>) {
    let text = format!(
        "version = 1\n[[rule]]\nid = \"s\"\nengine = \"stylelint\"\ncheck = \"{}\"\nfix = \"f\"\n{}",
        row.check, row.extra
    );
    let folder = PathBuf::from(format!("{ROOT}/{}", row.folder));
    let file = RuleFile::parse(&text, &folder).unwrap();
    let runs = RefCell::new(Vec::new());
    let sups = RefCell::new(Vec::new());
    let run = |cmd: &Cmd| {
        assert_eq!((cmd.program.to_str(), cmd.cwd.as_deref()), (Some("stylelint"), Some(folder.as_path())));
        let args = cmd.args.iter().map(|arg| arg.to_string_lossy().into_owned());
        let args: Vec<String> = args
            .map(|arg| match arg.strip_prefix("--suppress-location=") {
                Some(path) => {
                    sups.borrow_mut().push((PathBuf::from(path), std::fs::read_to_string(path).ok()));
                    SUP.to_string()
                }
                None => arg,
            })
            .collect();
        let probing = args[0] == "--print-config";
        runs.borrow_mut().push(args);
        Ok(match probing {
            true => Output { code: Some(0), stdout: row.probe.to_vec(), stderr: Vec::new() },
            false => Output { code: Some(row.answer.0), stdout: Vec::new(), stderr: row.answer.1.to_vec() },
        })
    };
    let on_path = |_: &str| -> bool { panic!("run_stylelint_with looks nothing up on PATH") };
    let raw_chars = Cell::new(None);
    let host = Host { run: &run, on_path: &on_path, raw_chars: &raw_chars, base: None };
    let sources = |at: &Path| {
        assert_eq!(at, folder);
        Ok(row.listed.iter().map(PathBuf::from).collect())
    };
    let got = run_stylelint_with(&sources, &STYLELINT, &file.rules[0], &folder, &host);
    (got, runs.into_inner(), raw_chars.get(), sups.into_inner())
}

#[test]
fn run_stylelint_with_probes_runs_and_settles() {
    let hex = "color-no-invalid-hex";
    let rows = [
        Row { name: "no file", check: hex, extra: "", folder: "two", listed: &[], probe: RULES, answer: (0, b""),
              want: Err(Stop::NoFiles { why: "no file its engine checks".into() }), runs: vec![], counted: false },
        Row { name: "the check off", check: "color-named", extra: "", folder: "pc", listed: &["a.css"], probe: RULES, answer: (0, b""),
              want: Err(Stop::NotAvailable { why: "stylelint's config does not turn on color-named for any file the rule covers".into() }),
              runs: vec![probe("./a.css")], counted: false },
        Row { name: "fix", check: "color-hex-length", extra: "", folder: "fix", listed: &["a.css"], probe: FIX, answer: (0, b""),
              want: Err(Stop::Failed { code: None, stderr: "stylelint's config for a.css turns on fix, which rewrites the checked files; cctao never edits the code it checks".into() }),
              runs: vec![probe("./a.css")], counted: false },
        Row { name: "findings", check: hex, extra: "paths = [\"a.css\", \"b.css\"]\nconfig = \"cfg.json\"\n", folder: "two",
              listed: &["a.css", "b.css", "lib/c.css"], probe: RULES, answer: (2, include_bytes!("../../stylelint/fixtures/findings.txt")),
              want: Ok(vec![
                  hit("a.css", 1, "Invalid hex color \"#fff1az\" (color-no-invalid-hex)"),
                  hit("b.css", 2, "Invalid hex color \"#12345g\" (color-no-invalid-hex)"),
              ]),
              runs: vec![
                  vec!["--print-config", "--config=cfg.json", "./a.css"],
                  vec!["--print-config", "--config=cfg.json", "./b.css"],
                  check_run(&["--config=cfg.json", SUP, "./a.css", "./b.css"]),
              ],
              counted: true },
        Row { name: "ignored", check: hex, extra: "", folder: "ign", listed: &["a.css"], probe: RULES, answer: (0, include_bytes!("../../stylelint/fixtures/ignored.txt")),
              want: Err(Stop::Failed { code: Some(0), stderr: "stylelint ignored a.css through ignoreFiles in its config; exceptions go in the rule file".into() }),
              runs: vec![probe("./a.css"), check_run(&[SUP, "./a.css"])], counted: true },
        Row { name: "unknown", check: "color-no-invalid-hexx", extra: "", folder: "unk", listed: &["a.css"], probe: RULES,
              answer: (2, include_bytes!("../../stylelint/fixtures/unknown.txt")),
              want: Err(Stop::UnknownCheck { said: "Unknown rule color-no-invalid-hexx. Did you mean color-no-invalid-hex?".into() }),
              runs: vec![probe("./a.css"), check_run(&[SUP, "./a.css"])], counted: true },
        Row { name: "a param", check: hex, extra: "params = { option = 1 }\n", folder: "two", listed: &["a.css"], probe: RULES, answer: (0, b""),
              want: Err(Stop::Failed { code: None, stderr: "stylelint param option: a stylelint rule takes no params".into() }),
              runs: vec![probe("./a.css")], counted: false },
    ];
    let mut failed = Vec::new();
    for row in &rows {
        let (got, runs, raw_chars, sups) = run_row(row);
        let checked = row.runs.iter().filter(|args| args.contains(&SUP)).count();
        let sups_ok = sups.len() == checked && sups.iter().all(|(path, held)| held.as_deref() == Some("{}") && !path.exists());
        if got != row.want || runs != row.runs || raw_chars.is_some() != row.counted || !sups_ok {
            failed.push((row.name, got, runs, raw_chars, sups));
        }
    }
    assert!(failed.is_empty(), "failed rows: {failed:#?}");
}

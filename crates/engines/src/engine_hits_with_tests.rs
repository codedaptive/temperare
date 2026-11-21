//! Unit tests for engine_hits_with: ShellCheck's script scope settles first, then a base-marked rule's missing base, both
//! before availability.
//! The base-present, no-script, and unmarked rows are regression guards; the off-path and no-adapter rows guard that
//! the missing base settles before the PATH lookup and the adapter check.

use crate::engine_hits_with::engine_hits_with;
use crate::host::Host;
use crate::types::Stop;
use procio::{Cmd, Output};
use rules::RuleFile;
use std::cell::{Cell, RefCell};
use std::path::{Path, PathBuf};

#[test]
fn base_contract_engine_hits() {
    let cases = [
        ("base present", "ruff", "E711", "only_with_base = true", Some("abc123"), true, Ok(vec![]), vec!["lookup ruff", "run ruff"]),
        ("no script", "shellcheck", "SC2086", "only_with_base = true", None, true, Err(Stop::NoFiles { why: "no file its engine checks".into() }), vec![]),
        ("unmarked", "ruff", "E711", "", None, true, Ok(vec![]), vec!["lookup ruff", "run ruff"]),
        ("base missing", "ruff", "E711", "only_with_base = true", None, true, Err(Stop::NotAvailable { why: "reports only what a change adds; run with --base".into() }), vec![]),
        ("base missing, off path", "ruff", "E711", "only_with_base = true", None, false, Err(Stop::NotAvailable { why: "reports only what a change adds; run with --base".into() }), vec![]),
        ("base missing, no adapter", "pylint", "C0114", "only_with_base = true", None, true, Err(Stop::NotAvailable { why: "reports only what a change adds; run with --base".into() }), vec![]),
    ];
    let mut wrong = Vec::new();
    for (name, engine, check, mark, base, found, expected, calls) in cases {
        let text = format!("version = 1\n[[rule]]\nid = \"r\"\nengine = \"{engine}\"\ncheck = \"{check}\"\nfix = \"f\"\n{mark}\n");
        let rule = RuleFile::parse(&text, Path::new(".")).unwrap().rules.remove(0);
        let seen = RefCell::new(Vec::new());
        let on_path = |program: &str| {
            seen.borrow_mut().push(format!("lookup {program}"));
            found
        };
        let run = |cmd: &Cmd| {
            seen.borrow_mut().push(format!("run {}", cmd.program.to_string_lossy()));
            Ok(Output { code: Some(0), stdout: b"[]".to_vec(), stderr: vec![] })
        };
        let host = Host { run: &run, on_path: &on_path, raw_chars: &Cell::new(None), base };
        let got = engine_hits_with(&rule, Ok(Path::new(".")), &host, &|_| Ok(vec![]));
        let seen = seen.into_inner();
        if got != expected || seen != calls {
            wrong.push(format!("{name}: got {got:?} after {seen:?}"));
        }
    }
    assert!(wrong.is_empty(), "{wrong:#?}");
}

#[test]
fn scripts_settle_before_shellcheck_availability_and_other_engines_skip_the_lister() {
    let cases = [
        ("empty", "shellcheck", "SC2086", "", Some(vec![]), false, "no file its engine checks"),
        ("outside", "shellcheck", "SC2086", "paths = [\"src/**\"]", Some(vec!["docs/x.sh"]), false, "no file its engine checks"),
        ("absent shellcheck", "shellcheck", "SC2086", "", Some(vec!["src/a.sh"]), true, "shellcheck"),
        ("absent ruff", "ruff", "E711", "", None, true, "ruff"),
    ];
    // The last column is the `NoFiles` why when nothing is looked up, else the program `NotOnPath` names.
    for (name, engine, check, scope, scripts, lookup, why) in cases {
        let text = format!("version = 1\n[[rule]]\nid = \"r\"\nengine = \"{engine}\"\ncheck = \"{check}\"\nfix = \"f\"\n{scope}\n");
        let rule = RuleFile::parse(&text, Path::new(".")).unwrap().rules.remove(0);
        let on_path = |program: &str| {
            assert!(lookup, "{name}: unexpected lookup of {program}");
            assert_eq!(program, engine, "{name}");
            false
        };
        let run = |cmd: &Cmd| panic!("{name}: nothing may run: {cmd:?}");
        let host = Host { run: &run, on_path: &on_path, raw_chars: &Cell::new(None), base: None };
        let list = |_: &Path| Ok(scripts.as_ref().expect("lister must not run").iter().map(|path| PathBuf::from(*path)).collect());
        let want = if lookup {
            Stop::NotOnPath { program: why.into() }
        } else {
            Stop::NoFiles { why: why.into() }
        };
        assert_eq!(engine_hits_with(&rule, Ok(Path::new(".")), &host, &list), Err(want), "{name}");
    }
}

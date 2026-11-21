//! Unit tests for engine_hits.rs: an absent engine stops before anything runs, with availability's why; Tessera is never
//! available; ruff and SwiftLint go to their entries' flows; an unresolved folder fails only an available
//! engine. The host is a fake; nothing runs.

use crate::engine_hits::engine_hits;
use crate::host::Host;
use crate::types::{Hit, Stop};
use procio::{Cmd, Output, ProcError};
use rules::RuleFile;
use std::cell::{Cell, RefCell};
use std::ffi::OsString;
use std::path::Path;

const FOLDER: &str = "/no/such/engines-zut-folder";

/// `engine_hits` for a one-rule file with `engine` and `check`, PATH answering `on_path`, and every run answering
/// garbage with exit code 2; also the programs the host was asked to run, in order.
pub(crate) fn hits_of(engine: &str, check: &str, on_path: bool) -> (Result<Vec<Hit>, Stop>, Vec<OsString>) {
    let text = format!("version = 1\n[[rule]]\nid = \"r\"\nengine = \"{engine}\"\ncheck = \"{check}\"\nfix = \"f\"\n");
    let file = RuleFile::parse(&text, Path::new(FOLDER)).unwrap();
    let ran = RefCell::new(Vec::new());
    let run = |cmd: &Cmd| -> Result<Output, ProcError> {
        ran.borrow_mut().push(cmd.program.clone());
        Ok(Output { code: Some(2), stdout: b"garbage".to_vec(), stderr: Vec::new() })
    };
    let lookup = |_: &str| on_path;
    let got =
        engine_hits(&file.rules[0], Ok(Path::new(FOLDER)), &Host { run: &run, on_path: &lookup, raw_chars: &Cell::new(None) });
    (got, ran.into_inner())
}

/// `engine_hits` for a one-rule file with `engine` and `check`, given an unresolved folder; PATH answers `on_path`
/// and any run panics.
fn hits_without_folder(engine: &str, check: &str, on_path: bool) -> Result<Vec<Hit>, Stop> {
    let text = format!("version = 1\n[[rule]]\nid = \"r\"\nengine = \"{engine}\"\ncheck = \"{check}\"\nfix = \"f\"\n");
    let file = RuleFile::parse(&text, Path::new(FOLDER)).unwrap();
    let run = |_: &Cmd| -> Result<Output, ProcError> { panic!("nothing runs without a folder") };
    let lookup = |_: &str| on_path;
    let unresolved = Stop::Failed { code: None, stderr: "no folder".to_string() };
    engine_hits(&file.rules[0], Err(&unresolved), &Host { run: &run, on_path: &lookup, raw_chars: &Cell::new(None) })
}

#[test]
fn an_unresolved_folder_fails_an_available_engine() {
    let got = hits_without_folder("ruff", "E711", true);
    assert_eq!(got, Err(Stop::Failed { code: None, stderr: "no folder".to_string() }));
}

#[test]
fn an_unresolved_folder_leaves_absent_engines_and_tessera_not_available() {
    assert_eq!(hits_without_folder("swiftlint", "force_cast", false), not_available("swiftlint not found on PATH"));
    assert_eq!(
        hits_without_folder("tessera", "slop", true),
        not_available("Tessera's slop records are not defined yet")
    );
}

pub(crate) fn not_available(why: &str) -> Result<Vec<Hit>, Stop> {
    Err(Stop::NotAvailable { why: why.to_string() })
}

#[test]
fn an_absent_engine_is_not_available_with_availabilitys_why_and_nothing_runs() {
    let cases = [
        ("clippy", "clippy::bool_comparison", "cargo-clippy not found on PATH"),
        ("swiftlint", "force_cast", "swiftlint not found on PATH"),
        ("ruff", "E711", "ruff not found on PATH"),
        ("ast-grep", "rules/x.yml", "ast-grep not found on PATH"),
    ];
    for (engine, check, why) in cases {
        let (got, ran) = hits_of(engine, check, false);
        assert_eq!(got, not_available(why), "{engine}");
        assert!(ran.is_empty(), "{engine} ran {ran:?}");
    }
}

#[test]
fn tessera_is_not_available_even_with_everything_on_path_and_runs_nothing() {
    let (got, ran) = hits_of("tessera", "slop", true);
    assert_eq!(got, not_available("Tessera's slop records are not defined yet"));
    assert!(ran.is_empty(), "ran {ran:?}");
}

#[test]
fn ruff_and_swiftlint_go_to_their_entries_flows() {
    for program in ["ruff", "swiftlint"] {
        let (got, ran) = hits_of(program, "E711", true);
        assert_eq!(ran, vec![OsString::from(program)]);
        assert!(matches!(got, Err(Stop::Failed { code: Some(2), .. })), "{program}: {got:?}");
    }
}

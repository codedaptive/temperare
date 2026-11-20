//! ZUT for engine_versions.rs: each distinct engine once, in first-appearance order, probed through a fake host only
//! when it is available and the folder resolved; nothing runs.

use crate::engine_versions::engine_versions;
use crate::host::Host;
use crate::types::EngineVersion;
use procio::{Cmd, Output, ProcError};
use rules::{Engine, RuleFile};
use std::cell::{Cell, RefCell};
use std::path::Path;

const FOLDER: &str = "/no/such/engines-zut-folder";

const RULES: &str = r#"version = 1
[[rule]]
id = "a"
engine = "ruff"
check = "E711"
fix = "f"
[[rule]]
id = "b"
engine = "swiftlint"
check = "force_cast"
fix = "f"
[[rule]]
id = "c"
engine = "ruff"
check = "F401"
fix = "f"
[[rule]]
id = "d"
engine = "tessera"
check = "slop"
fix = "f"
[[rule]]
id = "e"
engine = "swiftlint"
check = "force_try"
fix = "f"
"#;

fn file() -> RuleFile {
    RuleFile::parse(RULES, Path::new(FOLDER)).unwrap()
}

/// A fake run that answers each probe by its program and records the programs it was asked to run.
fn probes<'a>(seen: &'a RefCell<Vec<String>>) -> impl Fn(&Cmd) -> Result<Output, ProcError> + 'a {
    move |cmd: &Cmd| {
        let program = cmd.program.to_string_lossy().to_string();
        let stdout = match program.as_str() {
            "ruff" => "ruff 0.17.4\n",
            "swiftlint" => "0.59.1\n",
            other => panic!("unexpected probe of {other}"),
        };
        seen.borrow_mut().push(program);
        Ok(Output { code: Some(0), stdout: stdout.as_bytes().to_vec(), stderr: vec![] })
    }
}

fn version(engine: Engine, version: Option<&str>) -> EngineVersion {
    EngineVersion { engine, version: version.map(str::to_string) }
}

#[test]
fn each_engine_once_in_first_appearance_order_with_one_probe_each() {
    let seen = RefCell::new(Vec::new());
    let run = probes(&seen);
    let host = Host { run: &run, on_path: &|_| true, raw_chars: &Cell::new(None) };
    assert_eq!(engine_versions(&file(), Some(Path::new(FOLDER)), &host), vec![
        version(Engine::Ruff, Some("0.17.4")),
        version(Engine::SwiftLint, Some("0.59.1")),
        version(Engine::Tessera, None),
    ]);
    assert_eq!(seen.take(), vec!["ruff", "swiftlint"]);
}

#[test]
fn an_absent_engine_and_tessera_are_none_with_no_probe_run() {
    let seen = RefCell::new(Vec::new());
    let run = probes(&seen);
    let host = Host { run: &run, on_path: &|program| program == "ruff", raw_chars: &Cell::new(None) };
    assert_eq!(engine_versions(&file(), Some(Path::new(FOLDER)), &host), vec![
        version(Engine::Ruff, Some("0.17.4")),
        version(Engine::SwiftLint, None),
        version(Engine::Tessera, None),
    ]);
    assert_eq!(seen.take(), vec!["ruff"]);
}

#[test]
fn an_unresolved_folder_gives_every_engine_none_with_no_probe_run() {
    let seen = RefCell::new(Vec::new());
    let run = probes(&seen);
    let host = Host { run: &run, on_path: &|_| true, raw_chars: &Cell::new(None) };
    assert_eq!(engine_versions(&file(), None, &host), vec![
        version(Engine::Ruff, None),
        version(Engine::SwiftLint, None),
        version(Engine::Tessera, None),
    ]);
    assert!(seen.take().is_empty());
}

#[test]
fn a_probe_that_fails_gives_none_for_its_engine_only() {
    let run = |cmd: &Cmd| -> Result<Output, ProcError> {
        if cmd.program == "ruff" {
            return Err(ProcError::NotFound { program: "ruff".into() });
        }
        Ok(Output { code: Some(0), stdout: b"0.59.1\n".to_vec(), stderr: vec![] })
    };
    let host = Host { run: &run, on_path: &|_| true, raw_chars: &Cell::new(None) };
    assert_eq!(engine_versions(&file(), Some(Path::new(FOLDER)), &host), vec![
        version(Engine::Ruff, None),
        version(Engine::SwiftLint, Some("0.59.1")),
        version(Engine::Tessera, None),
    ]);
}

#[test]
fn the_probes_leave_the_count_untouched() {
    let seen = RefCell::new(Vec::new());
    let run = probes(&seen);
    let raw_chars = Cell::new(None);
    let host = Host { run: &run, on_path: &|_| true, raw_chars: &raw_chars };
    engine_versions(&file(), Some(Path::new(FOLDER)), &host);
    assert_eq!(raw_chars.get(), None);
}

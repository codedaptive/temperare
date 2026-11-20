//! ZIT for temp_config.rs on real temp files: the config is written with the params' text and passed as one
//! argument, and removed when dropped; through run_plain, it is there during the run and gone after it.

use crate::entry::entry_cmd_tests::fake_entry;
use crate::entry::{run_plain, temp_config, ConfigParams, Entry, Params, Pick};
use crate::host::Host;
use crate::ParamError;
use procio::{Cmd, Output, ProcError};
use rules::RuleFile;
use serde_json::{json, Map, Value};
use std::cell::{Cell, RefCell};
use std::ffi::OsString;
use std::path::{Path, PathBuf};

/// `width = <n>\n` from the params' `width` number.
fn width_text(params: &Map<String, Value>) -> Result<String, ParamError> {
    let bad = || ParamError::Bad { name: "width".to_string(), why: "must be a number".to_string() };
    let width = params.get("width").and_then(Value::as_u64).ok_or_else(bad)?;
    Ok(format!("width = {width}\n"))
}

fn configured() -> Entry {
    let config = ConfigParams { option: "--settings=", ext: "toml", text: width_text };
    fake_entry(Pick::Arg("--select="), Params::Config(config), None)
}

#[test]
fn the_config_holds_the_params_text_is_passed_as_one_argument_and_is_removed_when_dropped() {
    let (temp, arg) = temp_config(&configured(), &json!({"width": 99})).unwrap().unwrap();
    assert_eq!(std::fs::read_to_string(&temp.path).unwrap(), "width = 99\n");
    let mut expected = OsString::from("--settings=");
    expected.push(&temp.path);
    assert_eq!(arg, expected);
    assert_eq!(temp.path.parent(), Some(std::env::temp_dir().as_path()));
    let name = temp.path.file_name().unwrap().to_str().unwrap().to_string();
    let prefix = format!("tm-lint-{}-", std::process::id());
    assert!(name.starts_with(&prefix) && name.ends_with(".toml"), "{name}");
    let path = temp.path.clone();
    drop(temp);
    assert!(!path.exists());
}

#[test]
fn run_plain_passes_the_config_and_removes_it_after_the_run() {
    let text = "version = 1\n[[rule]]\nid = \"r\"\nengine = \"ruff\"\ncheck = \"E711\"\nfix = \"f\"\nparams = { width = 80 }\n";
    let rule = RuleFile::parse(text, Path::new("/no/such/folder")).unwrap().rules.remove(0);
    let seen = RefCell::new(Vec::new());
    let run = |cmd: &Cmd| -> Result<Output, ProcError> {
        let args: Vec<&str> = cmd.args.iter().map(|arg| arg.to_str().unwrap()).collect();
        assert_eq!(args.len(), 4, "{args:?}");
        assert_eq!(args[..2], ["check", "--select=E711"]);
        assert_eq!(args[3], ".");
        let path = PathBuf::from(args[2].strip_prefix("--settings=").unwrap());
        seen.borrow_mut().push((std::fs::read_to_string(&path).ok(), path));
        Ok(Output { code: Some(0), stdout: Vec::new(), stderr: Vec::new() })
    };
    let host = Host { run: &run, on_path: &|_| true, raw_chars: &Cell::new(None) };
    let got = run_plain(&configured(), &rule, Path::new("/no/such/folder"), &host);
    assert_eq!(got, Ok(Vec::new()));
    let seen = seen.into_inner();
    assert_eq!(seen.len(), 1);
    let (during, path) = &seen[0];
    assert_eq!(during.as_deref(), Some("width = 80\n"));
    assert!(!path.exists(), "config remains at {}", path.display());
}

//! Unit test for actionlint_entry.rs: what the entry's fields drive. The command `entry_cmd` builds (the flow then adds the
//! config, the integrations and the files), the program it needs, the version read from actionlint 1.7.12's whole
//! `-version` stdout as fixtures/fixtures.txt records it, the refusal and the kind reader wired in, and a param refused
//! by `param_args`. Nothing runs.

use super::ACTIONLINT;
use crate::entry::{entry_cmd, param_args};
use crate::ParamError;
use procio::Output;
use rules::Engine;
use serde_json::json;
use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};

#[test]
fn actionlint_entry_drives_the_command_version_and_refusal() {
    assert_eq!((ACTIONLINT.name, ACTIONLINT.engine, ACTIONLINT.program), ("actionlint", Engine::Actionlint, "actionlint"));
    assert_eq!(ACTIONLINT.needs, ["actionlint"]);
    assert_eq!(ACTIONLINT.never, None);
    let cmd = entry_cmd(&ACTIONLINT, OsStr::new("expression"), &[], Some("ignored.yaml"), Path::new("/w/repo"));
    assert_eq!(cmd.program, OsString::from("actionlint"));
    let args = ["-no-color", "-format", r#"{"kinds":{{json allKinds}},"errors":{{json .}}}"#];
    assert_eq!(cmd.args, args.map(OsString::from));
    assert_eq!(cmd.cwd, Some(PathBuf::from("/w/repo")));
    let probe = ACTIONLINT.version.unwrap();
    assert_eq!(probe.args, ["-version"]);
    let stdout = b"1.7.12\ninstalled from Homebrew\nbuilt with go1.27.1 compiler for darwin/arm64\n".to_vec();
    assert_eq!((probe.read)(&Output { code: Some(0), stdout, stderr: vec![] }), Some("1.7.12".to_string()));
    assert!((ACTIONLINT.refuse)("Expression").is_some());
    let out = Output { code: Some(0), stdout: include_bytes!("fixtures/clean.json").to_vec(), stderr: vec![] };
    assert!((ACTIONLINT.unknown)(&out, "expresion").is_some());
    assert!(matches!(param_args(&ACTIONLINT, &json!({"ignore": ["x"]})), Err(ParamError::Bad { name, .. }) if name == "ignore"));
}

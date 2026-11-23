//! ZUT for stylelint_entry.rs: the fields of `STYLELINT` that carry the contract, through the command `entry_cmd` builds
//! from it (the check is not passed: `run_stylelint_with` appends the suppressions file and the files), its version
//! probe on stylelint 18.1.0's `--version` stdout (fixtures/fixtures.txt), the refusal and unknown reader wired in, and
//! a param refused by `param_args` naming the engine.

use super::STYLELINT;
use crate::entry::{entry_cmd, param_args, Pick};
use crate::ParamError;
use procio::Output;
use rules::Engine;
use serde_json::json;
use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};

#[test]
fn stylelint_entry_builds_the_command_and_reads_the_version() {
    assert_eq!((STYLELINT.name, STYLELINT.engine, STYLELINT.program), ("stylelint", Engine::Stylelint, "stylelint"));
    assert_eq!((STYLELINT.needs, STYLELINT.never, STYLELINT.pick), (&["stylelint"][..], None, Pick::None));
    let cmd = entry_cmd(&STYLELINT, OsStr::new("color-named"), &[], Some("cfg/.stylelintrc.json"), Path::new("/w/css"));
    let want = [
        "--formatter",
        "json",
        "--ignore-disables",
        "--ignore-path=/dev/null",
        "--disable-default-ignores",
        "--allow-empty-input",
        "--no-cache",
        "--config=cfg/.stylelintrc.json",
    ];
    assert_eq!((cmd.program, cmd.cwd), (OsString::from("stylelint"), Some(PathBuf::from("/w/css"))));
    assert_eq!(cmd.args, want.map(OsString::from));
    assert!(cmd.env.is_empty() && cmd.env_remove.is_empty(), "{:?} {:?}", cmd.env, cmd.env_remove);
    let probe = STYLELINT.version.unwrap();
    let out = Output { code: Some(0), stdout: b"18.1.0\n".to_vec(), stderr: Vec::new() };
    assert_eq!((probe.args, (probe.read)(&out)), (&["--version"][..], Some("18.1.0".to_string())));
    assert!((STYLELINT.refuse)("CssSyntaxError").is_some());
    let unknown = Output { code: Some(2), stdout: Vec::new(), stderr: include_bytes!("fixtures/unknown.txt").to_vec() };
    let said = "Unknown rule color-no-invalid-hexx. Did you mean color-no-invalid-hex?".to_string();
    assert_eq!((STYLELINT.unknown)(&unknown, "color-no-invalid-hexx"), Some(said));
    let why = "a stylelint rule takes no params".to_string();
    assert_eq!(param_args(&STYLELINT, &json!({"option": 1})), Err(ParamError::Bad { name: "option".to_string(), why }));
}

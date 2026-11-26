//! ZUT for markdownlint_entry.rs: what the entry's fields drive. The command `entry_cmd` builds (the flow then adds
//! the options module and the files), the programs it needs, the version probe wired to `--help` and
//! `markdownlint_version` (whose reading markdownlint_version_tests.rs proves), the refusal wired in, and a param
//! refused by `param_args` naming the engine. Nothing runs.

use super::{markdownlint_version, MARKDOWNLINT};
use crate::entry::{entry_cmd, param_args};
use crate::ParamError;
use procio::Output;
use rules::Engine;
use serde_json::json;
use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};

#[test]
fn markdownlint_entry_drives_the_command_version_and_refusal() {
    let entry = &MARKDOWNLINT;
    assert_eq!((entry.name, entry.engine, entry.program), ("markdownlint", Engine::Markdownlint, "markdownlint-cli2"));
    assert_eq!(entry.needs, ["markdownlint-cli2", "node"]);
    assert_eq!(entry.never, None);
    let cmd = entry_cmd(entry, OsStr::new("MD013"), &[], Some("ignored.json"), Path::new("/w/docs"));
    assert_eq!(cmd.program, OsString::from("markdownlint-cli2"));
    assert_eq!(cmd.args, [OsString::from("--no-globs")]);
    assert_eq!(cmd.cwd, Some(PathBuf::from("/w/docs")));
    assert!(cmd.env.is_empty() && cmd.env_remove.is_empty());
    let probe = entry.version.unwrap();
    assert_eq!(probe.args, ["--help"]);
    assert!(std::ptr::fn_addr_eq(probe.read, markdownlint_version as fn(&Output) -> Option<String>));
    assert!((entry.refuse)("MD 013").is_some());
    assert_eq!((entry.refuse)("line-length"), None);
    let why = "a markdownlint rule takes no params".to_string();
    assert_eq!(param_args(entry, &json!({"line_length": ["100"]})), Err(ParamError::Bad { name: "line_length".to_string(), why }));
}

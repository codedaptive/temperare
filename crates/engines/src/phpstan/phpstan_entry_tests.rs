//! Unit test for phpstan_entry.rs: what the entry's fields drive. The command `entry_cmd` builds for a level (the flow then
//! adds the wrapper config and the files), the programs it needs, the version read from PHPStan 2.3.7's whole
//! `--version` stdout as fixtures/fixtures.txt records it, the refusal wired in, and a param refused by `param_args`
//! naming the engine. Nothing runs.

use super::PHPSTAN;
use crate::entry::{entry_cmd, param_args};
use crate::ParamError;
use procio::Output;
use rules::Engine;
use serde_json::json;
use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};

#[test]
fn phpstan_entry_drives_the_command_version_and_refusal() {
    assert_eq!((PHPSTAN.name, PHPSTAN.engine, PHPSTAN.program), ("phpstan", Engine::Phpstan, "phpstan"));
    assert_eq!(PHPSTAN.needs, ["phpstan", "php"]);
    assert_eq!(PHPSTAN.never, None);
    let cmd = entry_cmd(&PHPSTAN, OsStr::new("max"), &[], Some("ignored.neon"), Path::new("/w/php"));
    assert_eq!(cmd.program, OsString::from("phpstan"));
    let args = ["analyse", "--no-progress", "--no-interaction", "--no-ansi", "--error-format=json", "--level=max"];
    assert_eq!(cmd.args, args.map(OsString::from));
    assert_eq!(cmd.cwd, Some(PathBuf::from("/w/php")));
    assert!(cmd.env.is_empty() && cmd.env_remove.is_empty());
    let probe = PHPSTAN.version.unwrap();
    assert_eq!(probe.args, ["--version"]);
    let out = Output { code: Some(0), stdout: b"PHPStan - PHP Static Analysis Tool 2.3.7\n".to_vec(), stderr: vec![] };
    assert_eq!((probe.read)(&out), Some("2.3.7".to_string()));
    assert!((PHPSTAN.refuse)("five").is_some());
    let why = "a phpstan rule takes no params".to_string();
    assert_eq!(param_args(&PHPSTAN, &json!({"level": ["5"]})), Err(ParamError::Bad { name: "level".to_string(), why }));
}

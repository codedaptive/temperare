//! ZUT for vale_entry.rs: what the entry's fields drive. The command `entry_cmd` builds (the flow then adds the
//! filter, the wrapper config and the files; the check and the rule's config are not passed), the programs it needs,
//! the version read from Vale 3.28.1's whole `--version` stdout as fixtures/fixtures.txt records it, the refusal and
//! the unknown reader wired in, and a param refused by `param_args` naming the engine. Nothing runs.

use super::VALE;
use crate::entry::{entry_cmd, param_args};
use crate::ParamError;
use procio::Output;
use rules::Engine;
use serde_json::json;
use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};

#[test]
fn vale_entry_drives_the_command_version_and_refusal() {
    assert_eq!((VALE.name, VALE.engine, VALE.program), ("vale", Engine::Vale, "vale"));
    assert_eq!(VALE.needs, ["vale"]);
    assert_eq!(VALE.never, None);
    let cmd = entry_cmd(&VALE, OsStr::new("Vale.Spelling"), &[], Some("ignored.ini"), Path::new("/w/docs"));
    assert_eq!(cmd.program, OsString::from("vale"));
    assert_eq!(cmd.args, ["--no-global", "--output=JSON"].map(OsString::from));
    assert_eq!(cmd.cwd, Some(PathBuf::from("/w/docs")));
    assert!(cmd.env.is_empty() && cmd.env_remove.is_empty());
    let probe = VALE.version.unwrap();
    assert_eq!(probe.args, ["--version"]);
    let out = Output { code: Some(0), stdout: b"vale version 3.28.1\n".to_vec(), stderr: vec![] };
    assert_eq!((probe.read)(&out), Some("3.28.1".to_string()));
    assert_eq!((VALE.refuse)("Spelling").as_deref(), Some("vale check `Spelling` is not a Vale rule name (Style.Rule, each part ASCII letters, digits, _ or -)"));
    assert_eq!((VALE.refuse)("Vale.Spelling"), None);
    let unknown = Output { code: Some(2), stdout: vec![], stderr: include_bytes!("fixtures/unknown.txt").to_vec() };
    assert_eq!((VALE.unknown)(&unknown, "Vale.Speling").as_deref(), Some("invalid operation: int(unknown) (1:86)\n | filter(Rules, {.Name == \"Vale.Speling\" || (none(Rules, {.Name == \"Vale.Speling\"}) && int(\"unknown\") == 0)})\n | .....................................................................................^"));
    let why = "a vale rule takes no params".to_string();
    assert_eq!(param_args(&VALE, &json!({"level": ["x"]})), Err(ParamError::Bad { name: "level".to_string(), why }));
}

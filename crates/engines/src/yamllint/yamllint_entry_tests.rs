//! ZUT for yamllint_entry.rs: what the entry's fields drive. The command `entry_cmd` builds for a check (the flow then
//! adds the `-d` data and the files; the check is not an argument), the program it needs, the version read from
//! yamllint 1.38.0's `--version` stdout as fixtures/fixtures.txt records it, the refusal wired in, and a param `refused`
//! fails naming the engine. Nothing runs.

use super::YAMLLINT;
use crate::entry::entry_cmd;
use crate::refused::refused;
use crate::types::Stop;
use procio::Output;
use rules::{Engine, RuleFile};
use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};

#[test]
fn yamllint_entry_drives_the_command_version_and_refusal() {
    assert_eq!((YAMLLINT.name, YAMLLINT.engine, YAMLLINT.program), ("yamllint", Engine::Yamllint, "yamllint"));
    assert_eq!(YAMLLINT.needs, ["yamllint"]);
    assert_eq!(YAMLLINT.never, None);
    let cmd = entry_cmd(&YAMLLINT, OsStr::new("truthy"), &[], Some("ignored.yaml"), Path::new("/w/yaml"));
    assert_eq!(cmd.program, OsString::from("yamllint"));
    assert_eq!(cmd.args, ["-f", "parsable"].map(OsString::from));
    assert_eq!(cmd.cwd, Some(PathBuf::from("/w/yaml")));
    let probe = YAMLLINT.version.unwrap();
    assert_eq!(probe.args, ["--version"]);
    let out = Output { code: Some(0), stdout: b"yamllint 1.38.0\n".to_vec(), stderr: vec![] };
    assert_eq!((probe.read)(&out), Some("1.38.0".to_string()));
    for (check, refused) in [("Truthy", true), ("truthy", false)] {
        assert_eq!((YAMLLINT.refuse)(check).is_some(), refused, "{check}");
    }
    let text = "version = 1\n[[rule]]\nid = \"t\"\nengine = \"yamllint\"\ncheck = \"truthy\"\nfix = \"f\"\nparams = { x = \"y\" }\n";
    let file = RuleFile::parse(text, Path::new("/w/yaml")).unwrap();
    let stderr = "yamllint param x: a yamllint rule takes no params".to_string();
    assert_eq!(refused(&file.rules[0]), Some(Stop::Failed { code: None, stderr }));
}

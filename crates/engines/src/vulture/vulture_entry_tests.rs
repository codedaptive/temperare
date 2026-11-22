use super::VULTURE;
use crate::entry::entry_cmd;
use procio::Output;
use rules::Engine;
use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};

#[test]
fn vulture_entry_builds_the_command_and_reads_the_version() {
    let folder = Path::new("/w/py");
    let cmd = entry_cmd(&VULTURE, OsStr::new("V103"), &[], Some("x.toml"), folder);
    assert_eq!((VULTURE.name, VULTURE.engine, VULTURE.program), ("vulture", Engine::Vulture, "vulture"));
    assert_eq!(VULTURE.needs, ["vulture"]);
    assert_eq!(VULTURE.never, None);
    assert_eq!(cmd.program, OsString::from("vulture"));
    assert_eq!(cmd.args, [OsString::from("--config=/dev/null"), OsString::from(".")]);
    assert_eq!(cmd.cwd, Some(PathBuf::from("/w/py")));
    let probe = VULTURE.version.unwrap();
    assert_eq!(probe.args, ["--version"]);
    let out = Output { code: Some(0), stdout: b"vulture 2.16\n".to_vec(), stderr: vec![] };
    assert_eq!((probe.read)(&out), Some("2.16".to_string()));
}

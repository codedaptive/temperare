use std::ffi::{OsStr, OsString};

use crate::effective_path::effective_path;
use crate::Cmd;

fn cmd(env: &[(&str, &str)], env_remove: &[&str]) -> Cmd {
    Cmd {
        env: env.iter().map(|(name, value)| (name.into(), value.into())).collect(),
        env_remove: env_remove.iter().map(OsString::from).collect(),
        ..Cmd::new("git")
    }
}

#[test]
fn the_path_the_child_would_see() {
    let rows: [(&str, Cmd, Option<&str>, Option<&str>); 6] = [
        ("neither: the process value", cmd(&[], &[]), Some("/usr/bin:/bin"), Some("/usr/bin:/bin")),
        ("env sets it", cmd(&[("PATH", "/opt/bin")], &[]), Some("/usr/bin"), Some("/opt/bin")),
        ("two env entries: last wins", cmd(&[("PATH", "/a"), ("PATH", "/b")], &[]), Some("/usr/bin"), Some("/b")),
        ("removed", cmd(&[], &["PATH"]), Some("/usr/bin"), None),
        ("removed and re-added", cmd(&[("PATH", "/opt/bin")], &["PATH"]), Some("/usr/bin"), Some("/opt/bin")),
        ("process none, nothing set", cmd(&[("HOME", "/h")], &["HOME"]), None, None),
    ];
    for (case, cmd, process, expected) in rows {
        let path = effective_path(&cmd, process.map(OsStr::new));
        assert_eq!(path, expected.map(OsString::from), "{case}");
    }
}

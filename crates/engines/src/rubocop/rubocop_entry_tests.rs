//! Unit test for rubocop_entry.rs: the entry's names, the command `entry_cmd` builds from `RUBOCOP` with and without a
//! config, and the version read from `rubocop --force-default-config --version`'s real output (`1.92.3`).

use super::RUBOCOP;
use crate::entry::entry_cmd;
use procio::{Cmd, Output};
use rules::Engine;
use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};

#[test]
fn rubocop_entry_builds_the_command_and_reads_the_version() {
    assert_eq!((RUBOCOP.name, RUBOCOP.engine, RUBOCOP.program), ("rubocop", Engine::Rubocop, "rubocop"));
    assert_eq!(RUBOCOP.needs, ["rubocop"]);
    assert_eq!(RUBOCOP.never, None);
    let fixed = ["--cache", "false", "--no-server", "--format", "json", "--ignore-disable-comments"];
    let cases: [(&str, Option<&str>, Vec<&str>); 2] = [
        ("no config", None, vec!["--only=Lint/UselessAssignment", "."]),
        ("a config", Some("ci/rubocop.yml"), vec!["--only=Lint/UselessAssignment", "--config=ci/rubocop.yml", "."]),
    ];
    let mut failed = Vec::new();
    for (name, config, tail) in cases {
        let result = std::panic::catch_unwind(|| {
            let cmd = entry_cmd(&RUBOCOP, OsStr::new("Lint/UselessAssignment"), &[], config, Path::new("/w/rb"));
            let args: Vec<OsString> = fixed.iter().chain(tail.iter()).map(OsString::from).collect();
            assert_eq!(cmd.program, OsString::from("rubocop"), "{name}");
            assert_eq!(cmd.args, args, "{name}");
            assert_eq!(cmd.cwd, Some(PathBuf::from("/w/rb")), "{name}");
        });
        if result.is_err() { failed.push(name); }
    }
    assert!(failed.is_empty(), "failed rows: {failed:?}");
    let probe = RUBOCOP.version.unwrap();
    assert_eq!(probe.args, ["--force-default-config", "--version"]);
    let out = Output { code: Some(0), stdout: b"1.92.3\n".to_vec(), stderr: vec![] };
    assert_eq!((probe.read)(&out), Some("1.92.3".to_string()));
    let cmd = (probe.wrap.expect("rubocop version probe must wrap its environment"))(Cmd::new("rubocop"));
    assert!(cmd.env.contains(&(OsString::from("RUBOCOP_OPTS"), OsString::from(""))));
    assert!(cmd.env.contains(&(OsString::from("https_proxy"), OsString::from("http://127.0.0.1:9"))));
}

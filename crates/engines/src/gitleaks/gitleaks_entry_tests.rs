//! ZUT for gitleaks_entry.rs: the command `entry_cmd` builds from `GITLEAKS`, the version read from gitleaks 8.30.1's
//! real `version` text, and a param refused by `GITLEAKS.run` before anything runs. Nothing runs.

use super::GITLEAKS;
use crate::entry::entry_cmd;
use crate::host::Host;
use crate::types::Stop;
use procio::{Cmd, Output};
use rules::RuleFile;
use std::cell::Cell;
use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};

const FIXED: [&str; 7] = ["dir", "--no-banner", "--no-color", "--redact", "--report-format=sarif", "--report-path=-", "--enable-rule=aws-access-token"];

#[test]
fn the_command_scans_the_folder_redacted_as_sarif() {
    let rows: [(&str, Option<&str>, &[&str]); 2] = [
        ("no config", None, &["."]),
        ("with config", Some("lint/gitleaks.toml"), &["--config=lint/gitleaks.toml", "."]),
    ];
    for (name, config, tail) in rows {
        let cmd = entry_cmd(&GITLEAKS, OsStr::new("aws-access-token"), &[], config, Path::new("/w/leak"));
        let want: Vec<OsString> = FIXED.iter().chain(tail).map(OsString::from).collect();
        assert_eq!(cmd.program, OsString::from("gitleaks"), "{name}");
        assert_eq!(cmd.args, want, "{name}");
        assert_eq!(cmd.cwd, Some(PathBuf::from("/w/leak")), "{name}");
    }
}

#[test]
fn the_version_is_read_from_gitleaks_version() {
    let probe = GITLEAKS.version.unwrap();
    assert_eq!(probe.args, ["version"]);
    let out = Output { code: Some(0), stdout: b"8.30.1\n".to_vec(), stderr: Vec::new() };
    assert_eq!((probe.read)(&out), Some("8.30.1".to_string()));
}

#[test]
fn a_param_is_failed_and_nothing_runs() {
    let text = "version = 1\n[[rule]]\nid = \"aws\"\nengine = \"gitleaks\"\ncheck = \"aws-access-token\"\nfix = \"f\"\nparams = { foo = 1 }\n";
    let file = RuleFile::parse(text, Path::new("/w/leak")).unwrap();
    let runs = Cell::new(0);
    let run = |_: &Cmd| {
        runs.set(runs.get() + 1);
        Ok(Output { code: Some(0), stdout: Vec::new(), stderr: Vec::new() })
    };
    let on_path = |_: &str| -> bool { panic!("gitleaks' run looks nothing up on PATH") };
    let host = Host { run: &run, on_path: &on_path, raw_chars: &Cell::new(None), base: None };
    let got = (GITLEAKS.run)(&GITLEAKS, &file.rules[0], Path::new("/w/leak"), &host);
    let stderr = "gitleaks param foo: a gitleaks rule takes no params".to_string();
    assert_eq!(got, Err(Stop::Failed { code: None, stderr }));
    assert_eq!(runs.get(), 0);
}

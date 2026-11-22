//! ZUT for pylint_entry.rs: the command `entry_cmd` builds from `PYLINT`, the version read from pylint 4.1.4's real
//! `--version` text, and a param refused by `run_plain(&PYLINT, ..)` before anything runs. Nothing runs.

use super::PYLINT;
use crate::entry::{entry_cmd, run_plain};
use crate::host::Host;
use crate::types::Stop;
use procio::{Cmd, Output};
use rules::RuleFile;
use std::cell::Cell;
use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};

#[test]
fn the_command_runs_one_message_as_json2_over_the_folder() {
    let rows: [(&str, Option<&str>, &[&str]); 2] = [
        (
            "no config",
            None,
            &["--persistent=n", "--output-format=json2", "--disable=all", "--enable=unknown-option-value,useless-option-value", "--enable=C0121", "."],
        ),
        (
            "with config",
            Some("lint/pylint.rc"),
            &[
                "--persistent=n",
                "--output-format=json2",
                "--disable=all",
                "--enable=unknown-option-value,useless-option-value",
                "--enable=C0121",
                "--rcfile=lint/pylint.rc",
                ".",
            ],
        ),
    ];
    for (name, config, want) in rows {
        let cmd = entry_cmd(&PYLINT, OsStr::new("C0121"), &[], config, Path::new("/w/py"));
        assert_eq!(cmd.program, OsString::from("pylint"), "{name}");
        assert_eq!(cmd.args, want.iter().map(OsString::from).collect::<Vec<_>>(), "{name}");
        assert_eq!(cmd.cwd, Some(PathBuf::from("/w/py")), "{name}");
        assert!(cmd.env.is_empty() && cmd.env_remove.is_empty(), "{name}");
    }
}

#[test]
fn the_version_is_read_from_pylint_dash_dash_version() {
    let probe = PYLINT.version.unwrap();
    assert_eq!(probe.args, ["--version"]);
    let stdout = b"pylint 4.1.4\nastroid 4.3.4\nPython 3.15.2 (main, Oct 25 2026, 16:57:51) [Clang 21.0.0 (clang-2100.3.34.2)]\n";
    let out = Output { code: Some(0), stdout: stdout.to_vec(), stderr: Vec::new() };
    assert_eq!((probe.read)(&out), Some("4.1.4".to_string()));
}

#[test]
fn a_param_is_failed_and_nothing_runs() {
    let text = "version = 1\n[[rule]]\nid = \"none\"\nengine = \"pylint\"\ncheck = \"C0121\"\nfix = \"f\"\nparams = { foo = 1 }\n";
    let file = RuleFile::parse(text, Path::new("/w/py")).unwrap();
    let runs = Cell::new(0);
    let run = |_: &Cmd| {
        runs.set(runs.get() + 1);
        Ok(Output { code: Some(0), stdout: Vec::new(), stderr: Vec::new() })
    };
    let on_path = |_: &str| -> bool { panic!("pylint's run looks nothing up on PATH") };
    let host = Host { run: &run, on_path: &on_path, raw_chars: &Cell::new(None), base: None };
    let got = run_plain(&PYLINT, &file.rules[0], Path::new("/w/py"), &host);
    let stderr = "pylint param foo: a pylint rule takes no params".to_string();
    assert_eq!(got, Err(Stop::Failed { code: None, stderr }));
    assert_eq!(runs.get(), 0);
}

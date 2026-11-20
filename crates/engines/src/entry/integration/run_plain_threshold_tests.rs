//! ZIT for run_plain.rs with a threshold on real temp files: a `File` reach's file is there during the run and
//! gone after it; a `Dir` reach's folder is in the command's environment during the run and gone after it.

use crate::entry::entry_cmd_tests::fake_entry;
use crate::entry::{run_plain, Entry, Params, Pick, Reach, Threshold};
use crate::host::Host;
use procio::{Cmd, Output, ProcError};
use rules::{Rule, RuleFile};
use std::cell::{Cell, RefCell};
use std::ffi::OsString;
use std::path::{Path, PathBuf};

const FOLDER: &str = "/no/such/folder";
static NESTING: [Threshold; 1] =
    [Threshold { name: "max_nesting", check: "nesting", max: 9, setting: |n| format!("max: {n}\n") }];

fn limited(reach: Reach) -> Entry {
    fake_entry(Pick::Arg("--only="), Params::Thresholds { args: &[], thresholds: &NESTING, reach }, None)
}

fn rule() -> Rule {
    let text = "version = 1\n[[rule]]\nid = \"r\"\nengine = \"swiftlint\"\ncheck = \"nesting\"\nfix = \"f\"\nparams = { max_nesting = 3 }\n";
    RuleFile::parse(text, Path::new(FOLDER)).unwrap().rules.remove(0)
}

/// Runs `run_plain` of `entry` once with a host that hands each command to `see`.
fn run_with(entry: &Entry, see: &dyn Fn(&Cmd)) {
    let run = |cmd: &Cmd| -> Result<Output, ProcError> {
        see(cmd);
        Ok(Output { code: Some(0), stdout: Vec::new(), stderr: Vec::new() })
    };
    let host = Host { run: &run, on_path: &|_| true, raw_chars: &Cell::new(None), base: None };
    assert_eq!(run_plain(entry, &rule(), Path::new(FOLDER), &host), Ok(Vec::new()));
}

#[test]
fn a_file_reachs_threshold_file_is_there_during_the_run_and_gone_after() {
    let seen = RefCell::new(Vec::new());
    run_with(&limited(Reach::File { option: "--config=", ext: "yml", own: None }), &|cmd| {
        let args: Vec<&str> = cmd.args.iter().map(|arg| arg.to_str().unwrap()).collect();
        assert_eq!(args.len(), 4, "{args:?}");
        assert_eq!(args[..2], ["check", "--only=nesting"]);
        assert_eq!(args[3], ".");
        let path = PathBuf::from(args[2].strip_prefix("--config=").unwrap());
        seen.borrow_mut().push((std::fs::read_to_string(&path).ok(), path));
    });
    let seen = seen.into_inner();
    assert_eq!(seen.len(), 1);
    let (during, path) = &seen[0];
    assert_eq!(during.as_deref(), Some("max: 3\n"));
    assert!(!path.exists(), "threshold file remains at {}", path.display());
}

#[test]
fn a_dir_reachs_folder_is_in_the_env_during_the_run_and_gone_after() {
    let seen = RefCell::new(Vec::new());
    run_with(&limited(Reach::Dir { env: "LINT_CONF_DIR", file: "lint.toml" }), &|cmd| {
        assert_eq!(cmd.args, ["check", "--only=nesting", "."].map(OsString::from).to_vec());
        assert_eq!(cmd.env.len(), 1, "{:?}", cmd.env);
        assert_eq!(cmd.env[0].0, OsString::from("LINT_CONF_DIR"));
        let dir = PathBuf::from(&cmd.env[0].1);
        seen.borrow_mut().push((std::fs::read_to_string(dir.join("lint.toml")).ok(), dir));
    });
    let seen = seen.into_inner();
    assert_eq!(seen.len(), 1);
    let (during, dir) = &seen[0];
    assert_eq!(during.as_deref(), Some("max: 3\n"));
    assert!(!dir.exists(), "threshold folder remains at {}", dir.display());
}

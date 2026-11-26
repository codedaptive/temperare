use super::{run_vulture_with::run_vulture_with, VULTURE};
use crate::host::Host;
use crate::types::{Hit, Stop};
use procio::{Cmd, Output};
use rules::{RelPath, RuleFile};
use std::cell::{Cell, RefCell};
use std::ffi::OsString;
use std::path::{Path, PathBuf};

#[test]
fn run_vulture_with_adds_exclusions_or_stops_before_running() {
    let cases = [
        ("nothing ignored", Ok(vec![]), Some(vec!["--config=/dev/null", "."])),
        ("ignored folder", Ok(vec![(PathBuf::from("/w/py/.venv"), true)]), Some(vec!["--config=/dev/null", "--exclude=/w/py/.venv/*", "."])),
        ("comma", Ok(vec![(PathBuf::from("/w/py/a,b.py"), false)]), None),
    ];
    let mut failed = Vec::new();
    for (name, entries, expected_args) in cases {
        let result = std::panic::catch_unwind(|| {
            let folder = Path::new("/w/py");
            let file = RuleFile::parse("version = 1\n[[rule]]\nid = \"dead\"\nengine = \"vulture\"\ncheck = \"V103\"\nfix = \"f\"\n", folder).unwrap();
            let seen = RefCell::new(Vec::new());
            let run = |cmd: &Cmd| {
                seen.borrow_mut().push(cmd.clone());
                Ok(Output { code: Some(3), stdout: b"example.py:1: unused function 'dead' (60% confidence)\n".to_vec(), stderr: vec![] })
            };
            let on_path = |_: &str| -> bool { panic!("run_vulture_with does not look up PATH") };
            let host = Host { run: &run, on_path: &on_path, raw_chars: &Cell::new(None), base: None };
            let ignored = |path: &Path| { assert_eq!(path, folder); entries.clone() };
            let got = run_vulture_with(&ignored, &VULTURE, &file.rules[0], folder, &host);
            let commands = seen.into_inner();
            if let Some(args) = expected_args {
                assert_eq!(got, Ok(vec![Hit { path: RelPath::new("example.py"), line: Some(1), col: None, what: "unused function 'dead'".to_string() }]));
                assert_eq!(commands.len(), 1, "{name}");
                assert_eq!(commands[0].args, args.iter().map(|arg| OsString::from(*arg)).collect::<Vec<_>>(), "{name}");
                assert!(commands[0].env.contains(&(OsString::from("PYTHONWARNINGS"), OsString::from("ignore"))), "{name}");
            } else {
                let Err(Stop::Failed { code: None, stderr }) = got else { panic!("{name}: {got:?}") };
                assert!(stderr.contains("/w/py/a,b.py"), "{name}: {stderr}");
                assert!(commands.is_empty(), "{name}: {commands:?}");
            }
        });
        if result.is_err() { failed.push(name); }
    }
    assert!(failed.is_empty(), "failed rows: {failed:?}");
}

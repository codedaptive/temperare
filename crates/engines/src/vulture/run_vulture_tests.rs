use super::{run_vulture, VULTURE};
use crate::host::Host;
use crate::types::Hit;
use procio::{Cmd, Output};
use rules::{RelPath, RuleFile};
use std::cell::{Cell, RefCell};
use std::ffi::OsString;
use std::path::Path;

#[test]
fn run_vulture_sets_warning_environment_and_reads_hits() {
    let folder = Path::new("/w/py");
    let file = RuleFile::parse("version = 1\n[[rule]]\nid = \"dead\"\nengine = \"vulture\"\ncheck = \"V103\"\nfix = \"f\"\n", folder).unwrap();
    let seen = RefCell::new(None);
    let run = |cmd: &Cmd| {
        *seen.borrow_mut() = Some(cmd.clone());
        Ok(Output { code: Some(3), stdout: b"example.py:1: unused function 'dead' (60% confidence)\n".to_vec(), stderr: vec![] })
    };
    let on_path = |_: &str| -> bool { panic!("run_vulture does not look up PATH") };
    let host = Host { run: &run, on_path: &on_path, raw_chars: &Cell::new(None), base: None };
    let got = run_vulture(&VULTURE, &file.rules[0], folder, &host);
    assert_eq!(got, Ok(vec![Hit { path: RelPath::new("example.py"), line: Some(1), col: None, what: "unused function 'dead'".to_string() }]));
    let cmd = seen.into_inner().unwrap();
    assert_eq!(cmd.args, [OsString::from("--config=/dev/null"), OsString::from(".")]);
    assert!(cmd.env.contains(&(OsString::from("PYTHONWARNINGS"), OsString::from("ignore"))));
}

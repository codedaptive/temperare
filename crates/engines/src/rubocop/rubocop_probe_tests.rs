//! Unit test for rubocop_probe.rs: the command a probe runs, its stdout on exit 0, and how it stops otherwise, on a fake
//! host. list_stock.txt is real rubocop output (see fixtures/fixtures.txt).

use super::rubocop_probe;
use crate::host::Host;
use crate::types::Stop;
use procio::{Cmd, Output, ProcError};
use std::cell::{Cell, RefCell};
use std::ffi::OsString;
use std::path::{Path, PathBuf};

const LIST: &str = include_str!("fixtures/list_stock.txt");

#[test]
fn rubocop_probe_runs_rubocop_in_the_folder_and_reads_its_stdout() {
    let cases: [(&str, fn() -> Result<Output, ProcError>, Result<String, Stop>); 3] = [
        ("exit 0", || Ok(Output { code: Some(0), stdout: LIST.as_bytes().to_vec(), stderr: b"noise".to_vec() }),
            Ok("Rakefile\na.rb\nlegacy/d.rb\nlib/b.rb\n".to_string())),
        ("exit 2", || Ok(Output { code: Some(2), stdout: b"{}".to_vec(), stderr: b"Error: bad config\n".to_vec() }),
            Err(Stop::Failed { code: Some(2), stderr: "Error: bad config\n".to_string() })),
        ("not found", || Err(ProcError::NotFound { program: "rubocop".into() }),
            Err(Stop::NotOnPath { program: "rubocop".to_string() })),
    ];
    let mut failed = Vec::new();
    for (name, answer, want) in cases {
        let result = std::panic::catch_unwind(|| {
            let seen = RefCell::new(Vec::new());
            let run = |cmd: &Cmd| {
                seen.borrow_mut().push(cmd.clone());
                answer()
            };
            let on_path = |_: &str| -> bool { panic!("rubocop_probe looks nothing up on PATH") };
            let raw_chars = Cell::new(None);
            let host = Host { run: &run, on_path: &on_path, raw_chars: &raw_chars, base: None };
            let got = rubocop_probe(&["--list-target-files", "--force-default-config", "."], Path::new("/w/rb"), &host);
            assert_eq!(got, want, "{name}");
            assert_eq!(raw_chars.get(), None, "{name}: a probe is not the counted check run");
            let commands = seen.into_inner();
            assert_eq!(commands.len(), 1, "{name}");
            assert_eq!(commands[0].program, OsString::from("rubocop"), "{name}");
            let args: Vec<OsString> = ["--list-target-files", "--force-default-config", "."].map(OsString::from).to_vec();
            assert_eq!(commands[0].args, args, "{name}");
            assert_eq!(commands[0].cwd, Some(PathBuf::from("/w/rb")), "{name}");
            assert!(commands[0].env.contains(&(OsString::from("RUBOCOP_OPTS"), OsString::new())), "{name}");
            assert!(commands[0].env.contains(&(OsString::from("https_proxy"), OsString::from("http://127.0.0.1:9"))), "{name}");
        });
        if result.is_err() { failed.push(name); }
    }
    assert!(failed.is_empty(), "failed rows: {failed:?}");
}

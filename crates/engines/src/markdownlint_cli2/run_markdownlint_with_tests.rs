//! ZUT for run_markdownlint_with.rs with a fake listing, filesystem and host: each stop before the run, with nothing
//! run, nothing counted and nothing written. The steps that write cctao's options module and run markdownlint-cli2 touch
//! the real temporary folder, so the zone integration test tests/markdownlint_cli2.rs proves them.

use super::{run_markdownlint_with, MARKDOWNLINT};
use crate::host::Host;
use crate::types::Stop;
use procio::{Cmd, Output, ProcError};
use rules::RuleFile;
use std::cell::{Cell, RefCell};
use std::io;
use std::path::{Path, PathBuf};

const FOLDER: &str = "/no/such/engines-zut-markdownlint";

#[test]
fn run_markdownlint_with_stops_before_the_run() {
    let git_failed = Stop::Failed { code: None, stderr: "not a working copy".into() };
    let off = ("docs/.markdownlint.json", "{\"MD013\": false}");
    let off_name = Path::new("docs").join(".markdownlint.json").display().to_string();
    let rows: [(&str, &str, &str, Result<Vec<&str>, Stop>, &[(&str, &str)], &str, &str); 7] = [
        ("rule config", "MD013", "config = \"x.json\"\n", Ok(vec!["a.md"]), &[], "failed", "takes no config"),
        ("listing error", "MD013", "", Err(git_failed), &[], "failed", "not a working copy"),
        ("no file in paths", "MD013", "paths = [\"docs/**\"]\n", Ok(vec!["a.md"]), &[], "no-files", "no file its engine checks"),
        ("options file", "MD013", "", Ok(vec!["a.md"]), &[(".markdownlint-cli2.yaml", "fix: true\n")], "failed",
            ".markdownlint-cli2.yaml"),
        ("unlisted check", "MD999", "", Ok(vec!["a.md"]), &[], "not-listed", "MD999"),
        ("config off", "MD013", "", Ok(vec!["docs/b.md"]), &[off], "not-available", off_name.as_str()),
        ("only files in paths reach the configs", "MD013", "paths = [\"docs/**\"]\n", Ok(vec!["other/a.md", "docs/b.md"]),
            &[("other/.markdownlint-cli2.jsonc", "{}"), off], "not-available", off_name.as_str()),
    ];
    for (label, check, extra, listed, present, kind, token) in rows {
        let text = format!("version = 1\n[[rule]]\nid = \"r\"\nengine = \"markdownlint\"\ncheck = \"{check}\"\nfix = \"f\"\n{extra}");
        let rule = RuleFile::parse(&text, Path::new(FOLDER)).unwrap().rules.remove(0);
        let present: Vec<(PathBuf, &str)> = present.iter().map(|(path, text)| (Path::new(FOLDER).join(path), *text)).collect();
        let list = |_: &Path| listed.clone().map(|files| files.into_iter().map(PathBuf::from).collect());
        let is_file = |path: &Path| present.iter().any(|(known, _)| known == path);
        let read = |path: &Path| {
            present.iter().find(|(known, _)| known == path).map(|(_, text)| text.to_string()).ok_or_else(|| io::Error::from(io::ErrorKind::NotFound))
        };
        let ran = RefCell::new(Vec::new());
        let host_run = |cmd: &Cmd| -> Result<Output, ProcError> {
            ran.borrow_mut().push(cmd.clone());
            Ok(Output { code: Some(0), stdout: b"[]".to_vec(), stderr: Vec::new() })
        };
        let raw_chars = Cell::new(None);
        let host = Host { run: &host_run, on_path: &|_| true, raw_chars: &raw_chars, base: None };
        let got = run_markdownlint_with(&list, &is_file, &read, &MARKDOWNLINT, &rule, Path::new(FOLDER), &host);
        let said = match (&got, kind) {
            (Err(Stop::Failed { code: _, stderr }), "failed") => stderr,
            (Err(Stop::NoFiles { why }), "no-files") => why,
            (Err(Stop::NotListed { said }), "not-listed") => said,
            (Err(Stop::NotAvailable { why }), "not-available") => why,
            _ => panic!("{label}: expected {kind}, got {got:?}"),
        };
        assert!(said.contains(token), "{label}: {token:?} not in {said:?}");
        assert!(ran.borrow().is_empty(), "{label}: ran {:?}", ran.borrow());
        assert_eq!(raw_chars.get(), None, "{label}");
    }
}

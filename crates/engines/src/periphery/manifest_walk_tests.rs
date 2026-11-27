//! ZUT for `manifest_walk` on a fake host: the root's manifest is dumped once even when it names itself, and a local
//! package at the repository top is named "." in a why. The repository top is the canonical filesystem root ("/" on
//! Unix, `\\?\C:\` on Windows), which exists on every host; canonicalizing it is the only disk touch.

use super::manifest_walk;
use crate::host::Host;
use crate::types::Stop;
use procio::{Cmd, Output};
use std::cell::{Cell, RefCell};
use std::path::Path;

/// `root` and each of `calls` name a folder under the canonical filesystem root, "" being the root itself.
fn check_walk(root: &str, expected: Result<(), Stop>, calls: &[&str]) {
    let top = std::fs::canonicalize("/").unwrap();
    let on_top = serde_json::json!({"dependencies": [{"fileSystem": [{"path": top}]}], "targets": []}).to_string();
    let seen = RefCell::new(Vec::new());
    let run = |cmd: &Cmd| {
        let cwd = cmd.cwd.as_deref().unwrap();
        seen.borrow_mut().push(cwd.to_path_buf());
        let stdout = match cwd {
            cwd if cwd == top.join("x") => on_top.as_bytes(),
            cwd if cwd == top && root.is_empty() => on_top.as_bytes(),
            cwd if cwd == top => br#"{"dependencies":[{"sourceControl":[{"identity":"dep"}]}],"targets":[]}"#.as_slice(),
            path => panic!("unexpected dump-package cwd {}", path.display()),
        };
        Ok(Output { code: Some(0), stdout: stdout.to_vec(), stderr: vec![] })
    };
    let on_path = |_: &str| true;
    let host = Host { run: &run, on_path: &on_path, raw_chars: &Cell::new(None), base: None };
    let got = manifest_walk(&top.join(root), &top, Path::new("/scratch"), &host);
    assert_eq!(seen.into_inner(), calls.iter().map(|path| top.join(path)).collect::<Vec<_>>());
    assert_eq!(got, expected);
}

macro_rules! cases {
    ($( $name:ident => ($root:expr, $expected:expr, $calls:expr) ),+ $(,)?) => {
        $(#[test] fn $name() { check_walk($root, $expected, $calls); })+
    };
}

cases! {
    periphery_tidy_self_dependency => ("", Ok(()), &[""]),
    periphery_tidy_top_local_remote => (
        "x",
        Err(Stop::NotAvailable { why: "the local package . depends on dep, which cctao would have to fetch: cctao never fetches packages, so Periphery cannot build it here".into() }),
        &["x", ""]
    ),
}

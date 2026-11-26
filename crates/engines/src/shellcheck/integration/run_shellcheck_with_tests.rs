//! The shellcheck flow against real shellcheck and throwaway git repositories.

use crate::entry::{Entry, Params, Pick, VersionProbe};
use crate::host::Host;
use crate::shellcheck::{parse_shellcheck, run_shellcheck_with, shell_files, shellcheck_refusal, shellcheck_version};
use crate::types::{Hit, Stop};
use procio::Cmd;
use rules::{Engine, RuleFile};
use std::cell::Cell;
use std::fs;
use std::path::{Path, PathBuf};

#[path = "../../../tests/support/git.rs"]
mod git_support;

const LIST: &str = "# source: test\n# version: test\nSC2086\n";

struct TempRepo(PathBuf);

impl TempRepo {
    fn new(tag: &str, file: &str, contents: &str) -> Self {
        let path = std::env::temp_dir().join(format!("engines-zit-{}-shellcheck-flow-{tag}", std::process::id()));
        fs::create_dir(&path).unwrap();
        let repo = Self(fs::canonicalize(path).unwrap());
        git(&repo.0, &["init", "-q"]);
        fs::write(repo.0.join(file), contents).unwrap();
        git(&repo.0, &["add", file]);
        repo
    }
}

impl Drop for TempRepo {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).unwrap();
    }
}

fn git(repo: &Path, args: &[&str]) {
    git_support::git(repo, args, b"");
}

fn check(repo: &Path) -> (Result<Vec<Hit>, Stop>, Option<usize>) {
    let entry = Entry {
        name: "shellcheck",
        engine: Engine::ShellCheck,
        program: "shellcheck",
        needs: &["shellcheck"],
        never: None,
        args: &[],
        pick: Pick::None,
        params: Params::Args(&[]),
        config: None,
        after: &[],
        version: Some(VersionProbe { args: &["--version"], read: shellcheck_version, wrap: None }),
        refuse: shellcheck_refusal,
        unknown: |_, _| None,
        read: parse_shellcheck,
        run: |_, _, _, _| Ok(Vec::new()),
    };
    let mut file = RuleFile::parse(
        "version = 1\n[[rule]]\nid = \"quote\"\nengine = \"shellcheck\"\ncheck = \"SC2086\"\nfix = \"Quote the variable.\"\n",
        repo,
    ).unwrap();
    let raw_chars = Cell::new(None);
    let host = Host { run: &procio::run, on_path: &|p| procio::which(p).is_some(), raw_chars: &raw_chars, base: None };
    let result = run_shellcheck_with(LIST, &shell_files, &entry, &file.rules.remove(0), repo, &host);
    (result, raw_chars.get())
}

#[test]
fn tracked_clean_script_has_no_hits_and_counts_output() {
    let repo = TempRepo::new("clean", "clean.sh", "#!/bin/sh\nname=\"${1}\"\necho \"${name}\"\n");
    let (result, raw_chars) = check(&repo.0);
    assert_eq!(result, Ok(vec![]));
    // shellcheck 0.11.0 prints `{"comments":[]}` and a newline, 16 characters, and nothing on stderr.
    assert_installed_version_is_0_11_0();
    assert_eq!(raw_chars, Some(16));
}

/// The clean script's count is shellcheck 0.11.0's output; another version fails here first, with a plain message.
fn assert_installed_version_is_0_11_0() {
    let out = procio::run(&Cmd { args: vec!["--version".into()], ..Cmd::new("shellcheck") }).unwrap();
    assert_eq!(shellcheck_version(&out).as_deref(), Some("0.11.0"), "the expected counts are shellcheck 0.11.0's");
}

#[test]
fn repository_without_scripts_is_no_files_and_no_check_run() {
    let repo = TempRepo::new("no-scripts", "notes.txt", "plain text\n");
    let (result, raw_chars) = check(&repo.0);
    assert_eq!(result, Err(Stop::NoFiles { why: "no file its engine checks".to_string() }));
    assert_eq!(raw_chars, None);
}

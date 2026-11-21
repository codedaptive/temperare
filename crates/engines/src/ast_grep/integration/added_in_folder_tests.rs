//! Zone integration tests for added_in_folder.rs: a throwaway git repository with a base commit and a change in the working tree.
//! Symlinks, the paths `keep` rejects and a base git cannot read are change_of's, tested in
//! src/integration/change_of_tests.rs.

use super::super::added_in_folder::added_in_folder;
use rules::RelPath;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

fn git(repo: &Path, args: &[&str]) {
    let out = procio::run(&procio::git_cmd(repo, args.iter().map(Into::into).collect())).unwrap();
    assert_eq!(out.code, Some(0), "git {args:?}: {}", String::from_utf8_lossy(&out.stderr));
}

/// A repository under the temp folder holding `files` in its one commit; returns its canonical path.
fn repo_with_base(test: &str, files: &[(&str, &str)]) -> PathBuf {
    let root = std::env::temp_dir().join(format!("engines-zit-{}-added-in-folder-{test}", std::process::id()));
    fs::create_dir(&root).unwrap();
    let root = fs::canonicalize(root).unwrap();
    git(&root, &["init", "-q"]);
    for (path, text) in files {
        write(&root, path, text);
    }
    git(&root, &["add", "-A"]);
    git(&root, &["-c", "user.name=t", "-c", "user.email=t@t", "commit", "-q", "-m", "base"]);
    root
}

fn write(root: &Path, path: &str, text: &str) {
    let full = root.join(path);
    fs::create_dir_all(full.parent().unwrap()).unwrap();
    fs::write(full, text).unwrap();
}

fn rel(entries: &[(&str, u32)]) -> BTreeMap<RelPath, u32> {
    entries.iter().map(|(path, line)| (RelPath::new(path), *line)).collect()
}

#[test]
fn the_change_gives_each_file_its_first_added_line() {
    let root = repo_with_base(
        "whole",
        &[("a.rs", "1\n2\n3\n4\n5\n6\n"), ("gone.rs", "1\n2\n3\n"), ("same.rs", "x\n")],
    );
    // a.rs: line 3 replaced and a line put after line 5, so its added lines are 3 and 6 (the lowest is 3).
    write(&root, "a.rs", "1\n2\nthree\n4\n5\nnew\n6\n");
    // gone.rs: only a line removed.
    write(&root, "gone.rs", "1\n3\n");
    write(&root, "fresh.rs", "fn f() {}\n");
    // empty.rs: untracked with no lines, so nothing is added.
    write(&root, "empty.rs", "");
    let got = added_in_folder(&root, "HEAD", &|_| true);
    fs::remove_dir_all(&root).unwrap();
    assert_eq!(got, Ok(rel(&[("a.rs", 3), ("fresh.rs", 1)])));
}

#[test]
fn a_subfolder_keys_its_files_relative_to_itself_and_drops_the_rest() {
    let root = repo_with_base("sub", &[("rules/src/a.rs", "1\n2\n"), ("other.rs", "1\n")]);
    write(&root, "rules/src/a.rs", "1\n2\n3\n");
    write(&root, "other.rs", "1\n2\n");
    write(&root, "rules/new.rs", "x\n");
    write(&root, "outside_new.rs", "x\n");
    let got = added_in_folder(&root.join("rules"), "HEAD", &|_| true);
    fs::remove_dir_all(&root).unwrap();
    assert_eq!(got, Ok(rel(&[("new.rs", 1), ("src/a.rs", 3)])));
}

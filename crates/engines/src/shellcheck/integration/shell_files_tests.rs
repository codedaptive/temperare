//! Shell file discovery against real throwaway git repositories.

use crate::shellcheck::shell_files;
use crate::types::Stop;
use std::fs;
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};

#[path = "../../../tests/support/git.rs"]
mod git_support;

fn tree(name: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!("engines-zit-{}-shell-files-{name}", std::process::id()));
    fs::create_dir(&root).unwrap();
    fs::canonicalize(root).unwrap()
}

fn git(repo: &Path, args: &[&str]) {
    git_support::git(repo, args, b"");
}

fn repo(name: &str) -> PathBuf {
    let root = tree(name);
    git(&root, &["init", "-q"]);
    root
}

fn assert_files(folder: &Path, expected: &[&str]) {
    let mut got = shell_files(folder).unwrap();
    let mut expected: Vec<PathBuf> = expected.iter().map(PathBuf::from).collect();
    got.sort();
    expected.sort();
    assert_eq!(got, expected);
}

#[test]
fn tracked_and_untracked_scripts_are_listed_but_ignored_and_plain_files_are_not() {
    let root = repo("tracked-untracked");
    fs::write(root.join("a.sh"), "echo a\n").unwrap();
    fs::write(root.join("run"), "#!/usr/bin/env bash\n").unwrap();
    fs::write(root.join("notes"), "plain\n").unwrap();
    fs::write(root.join(".gitignore"), "i.sh\n").unwrap();
    git(&root, &["add", "a.sh", "run", "notes", ".gitignore"]);
    fs::write(root.join("u.sh"), "echo u\n").unwrap();
    fs::write(root.join("i.sh"), "echo ignored\n").unwrap();
    assert_files(&root, &["a.sh", "run", "u.sh"]);
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn tracked_file_deleted_from_disk_is_not_listed() {
    let root = repo("deleted");
    fs::write(root.join("d.sh"), "echo d\n").unwrap();
    git(&root, &["add", "d.sh"]);
    fs::remove_file(root.join("d.sh")).unwrap();
    assert_files(&root, &[]);
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn tracked_symlink_is_excluded_but_its_tracked_target_is_listed() {
    let root = repo("tracked-link");
    fs::write(root.join("a.sh"), "echo a\n").unwrap();
    symlink("a.sh", root.join("link.sh")).unwrap();
    git(&root, &["add", "a.sh", "link.sh"]);
    assert_files(&root, &["a.sh"]);
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn untracked_symlink_is_excluded_but_its_tracked_target_is_listed() {
    let root = repo("untracked-link");
    fs::write(root.join("a.sh"), "echo a\n").unwrap();
    git(&root, &["add", "a.sh"]);
    symlink("a.sh", root.join("link.sh")).unwrap();
    assert_files(&root, &["a.sh"]);
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn extensionless_symlink_is_excluded_but_its_shebang_target_is_listed() {
    let root = repo("extensionless-link");
    fs::write(root.join("run"), "#!/usr/bin/env bash\n").unwrap();
    symlink("run", root.join("runlink")).unwrap();
    git(&root, &["add", "run", "runlink"]);
    assert_files(&root, &["run"]);
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn subfolder_paths_are_relative_and_exclude_outside_targets_and_symlinks() {
    let root = repo("subfolder");
    fs::create_dir(root.join("sub")).unwrap();
    fs::write(root.join("sub/s.sh"), "echo s\n").unwrap();
    fs::write(root.join("top.sh"), "echo top\n").unwrap();
    git(&root, &["add", "sub/s.sh", "top.sh"]);
    fs::write(root.join("sub/t.sh"), "echo t\n").unwrap();
    symlink("../top.sh", root.join("sub/out.sh")).unwrap();
    assert_files(&fs::canonicalize(root.join("sub")).unwrap(), &["s.sh", "t.sh"]);
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn a_tracked_script_under_a_symlinked_folder_is_not_listed() {
    let root = tree("tracked-symlinked-folder");
    fs::create_dir(root.join("repo")).unwrap();
    fs::create_dir(root.join("outside")).unwrap();
    let repo = fs::canonicalize(root.join("repo")).unwrap();
    git(&repo, &["init", "-q"]);
    fs::create_dir(repo.join("sub")).unwrap();
    fs::write(repo.join("sub/x.sh"), "echo tracked\n").unwrap();
    fs::write(repo.join("a.sh"), "echo a\n").unwrap();
    git(&repo, &["add", "sub/x.sh", "a.sh"]);
    fs::remove_file(repo.join("sub/x.sh")).unwrap();
    fs::remove_dir(repo.join("sub")).unwrap();
    fs::write(root.join("outside/x.sh"), "echo outside\n").unwrap();
    symlink("../outside", repo.join("sub")).unwrap();
    assert_files(&repo, &["a.sh"]);
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn non_repository_returns_failed_not_not_available() {
    let root = tree("not-repo");
    let result = shell_files(&root);
    fs::remove_dir_all(root).unwrap();
    assert!(result.is_err());
    assert!(matches!(result, Err(Stop::Failed { .. })), "{result:?}");
}

#[test]
fn skip_worktree_script_is_not_listed_even_when_present() {
    let root = repo("skip-worktree");
    fs::write(root.join("skip.sh"), "echo skip\n").unwrap();
    git(&root, &["add", "skip.sh"]);
    git(&root, &["update-index", "--skip-worktree", "skip.sh"]);
    assert_files(&root, &[]);
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn unmerged_script_is_listed_once() {
    let root = repo("unmerged");
    fs::write(root.join("conflict.sh"), "echo conflict\n").unwrap();
    let hash = git_support::git(&root, &["hash-object", "-w", "--stdin"], b"echo conflict\n");
    let hash = std::str::from_utf8(&hash).unwrap().trim();
    let entries = format!("100644 {hash} 1\tconflict.sh\n100644 {hash} 2\tconflict.sh\n100644 {hash} 3\tconflict.sh\n");
    git_support::git(&root, &["update-index", "--index-info"], entries.as_bytes());
    assert_files(&root, &["conflict.sh"]);
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn recognized_shell_extensions_and_extensionless_shebangs_are_listed() {
    let root = repo("extensions");
    for name in ["a.sh", "b.bash", "c.ksh", "d.dash"] {
        fs::write(root.join(name), "plain\n").unwrap();
    }
    for (name, shebang) in [("sh-run", "#!/bin/sh\n"), ("bash-run", "#!/bin/bash\n"),
                            ("ksh-run", "#!/bin/ksh\n"), ("dash-run", "#!/bin/dash\n")] {
        fs::write(root.join(name), shebang).unwrap();
    }
    fs::write(root.join("other.txt"), "#!/bin/sh\n").unwrap();
    git(&root, &["add", "."]);
    assert_files(&root, &["a.sh", "b.bash", "c.ksh", "d.dash", "sh-run", "bash-run", "ksh-run", "dash-run"]);
    fs::remove_dir_all(root).unwrap();
}

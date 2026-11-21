//! Unit tests for shell_files_with.rs with fake probes: which listed paths are kept, by folder, regular file, extension and
//! shebang, and which paths each probe is asked about. No git runs and no file is read.

use super::shell_files_with;
use std::cell::RefCell;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

const TOP: &str = "/no/such/engines-zut-top";

/// What one call gives: the paths kept, and the full paths `regular` and `head` were asked about, in order.
struct Listed {
    got: Vec<PathBuf>,
    regular_asked: Vec<PathBuf>,
    head_asked: Vec<PathBuf>,
}

/// `shell_files_with` on `listed` under `folder`, with `TOP/name` a regular file for each name in `regular` and
/// `heads[name]` its first bytes.
fn list(folder: &str, listed: &[&str], regular: &[&str], heads: &[(&str, &[u8])]) -> Listed {
    let top = Path::new(TOP);
    let regular: Vec<PathBuf> = regular.iter().map(|name| top.join(name)).collect();
    let heads: HashMap<PathBuf, Vec<u8>> = heads.iter().map(|(name, bytes)| (top.join(name), bytes.to_vec())).collect();
    let regular_asked = RefCell::new(Vec::new());
    let head_asked = RefCell::new(Vec::new());
    let is_regular = |path: &Path| {
        regular_asked.borrow_mut().push(path.to_path_buf());
        regular.iter().any(|known| known == path)
    };
    let head = |path: &Path| {
        head_asked.borrow_mut().push(path.to_path_buf());
        heads.get(path).cloned()
    };
    let listed: Vec<PathBuf> = listed.iter().map(PathBuf::from).collect();
    let got = shell_files_with(Path::new(folder), top, &listed, &is_regular, &head);
    Listed { got, regular_asked: regular_asked.into_inner(), head_asked: head_asked.into_inner() }
}

fn paths(names: &[&str]) -> Vec<PathBuf> {
    names.iter().map(PathBuf::from).collect()
}

#[test]
fn an_untracked_script_listed_after_the_tracked_ones_is_kept() {
    let got = list(TOP, &["a.sh", "u.sh"], &["a.sh", "u.sh"], &[]).got;
    assert_eq!(got, paths(&["a.sh", "u.sh"]));
}

#[test]
fn a_symlinked_script_is_dropped_and_its_target_kept() {
    let listed = list(TOP, &["a.sh", "link.sh"], &["a.sh"], &[]);
    assert_eq!(listed.got, paths(&["a.sh"]));
    assert_eq!(listed.regular_asked, paths(&["/no/such/engines-zut-top/a.sh", "/no/such/engines-zut-top/link.sh"]));
}

#[test]
fn an_extensionless_symlink_is_dropped_and_its_head_never_read() {
    let listed = list(TOP, &["runlink"], &[], &[("runlink", b"#!/bin/sh\n")]);
    assert_eq!(listed.got, Vec::<PathBuf>::new());
    assert_eq!(listed.head_asked, Vec::<PathBuf>::new());
}

#[test]
fn a_path_outside_the_folder_is_dropped_and_not_probed() {
    let listed = list("/no/such/engines-zut-top/sub", &["other/x.sh", "sub/s.sh"], &["other/x.sh", "sub/s.sh"], &[]);
    assert_eq!(listed.got, paths(&["s.sh"]));
    assert_eq!(listed.regular_asked, paths(&["/no/such/engines-zut-top/sub/s.sh"]));
}

#[test]
fn a_vanished_path_is_dropped() {
    let got = list(TOP, &["gone.sh", "a.sh"], &["a.sh"], &[]).got;
    assert_eq!(got, paths(&["a.sh"]));
}

#[test]
fn an_extensionless_file_with_a_shell_shebang_is_kept() {
    let listed = list(TOP, &["run"], &["run"], &[("run", b"#!/usr/bin/env bash\necho run\n")]);
    assert_eq!(listed.got, paths(&["run"]));
    assert_eq!(listed.head_asked, paths(&["/no/such/engines-zut-top/run"]));
}

#[test]
fn a_shebang_of_another_shell_is_dropped() {
    let got = list(TOP, &["run"], &["run"], &[("run", b"#!/bin/zsh\n")]).got;
    assert_eq!(got, Vec::<PathBuf>::new());
}

#[test]
fn a_file_head_cannot_read_is_dropped() {
    let got = list(TOP, &["run"], &["run"], &[]).got;
    assert_eq!(got, Vec::<PathBuf>::new());
}

#[test]
fn only_the_shell_extensions_are_kept_and_their_heads_are_not_read() {
    let names = ["a.bash", "b.ksh", "c.dash", "d.shx", "e.py"];
    let listed = list(TOP, &names, &names, &[]);
    assert_eq!(listed.got, paths(&["a.bash", "b.ksh", "c.dash"]));
    assert_eq!(listed.head_asked, Vec::<PathBuf>::new());
}

#[test]
fn paths_come_back_relative_to_the_folder() {
    let got = list("/no/such/engines-zut-top/sub", &["sub/deep/d.sh", "sub/e.sh"], &["sub/deep/d.sh", "sub/e.sh"], &[]).got;
    assert_eq!(got, paths(&["deep/d.sh", "e.sh"]));
}

#[test]
fn nothing_listed_gives_none() {
    let listed = list(TOP, &[], &[], &[]);
    assert_eq!(listed.got, Vec::<PathBuf>::new());
    assert_eq!(listed.regular_asked, Vec::<PathBuf>::new());
}

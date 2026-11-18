//! ZIT for opening ast-grep rules without following symlinks.
#![cfg(unix)]

use super::super::open_in_folder::open_in_folder;
use std::fs;
use std::io::Read;
use std::os::unix::fs::symlink;
use std::path::PathBuf;

struct Tree {
    root: PathBuf,
    folder: PathBuf,
}

impl Tree {
    fn new(test: &str) -> Self {
        let root = std::env::temp_dir().join(format!("engines-zit-{}-open-in-folder-{test}", std::process::id()));
        fs::create_dir(&root).unwrap();
        let root = fs::canonicalize(root).unwrap();
        let folder = root.join("rules");
        fs::create_dir(&folder).unwrap();
        Self { root, folder }
    }
}

impl Drop for Tree {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.root).unwrap();
    }
}

#[test]
fn symlinked_file_to_outside_is_refused() {
    let tree = Tree::new("symlinked-file");
    fs::write(tree.root.join("outside.yml"), "id: outside\n").unwrap();
    symlink(tree.root.join("outside.yml"), tree.folder.join("names.yml")).unwrap();
    // The swap after rule_yaml_path's check is nondeterministic, so open the swapped state directly.
    assert_eq!(
        open_in_folder(&tree.folder, &tree.folder.join("names.yml"), "names.yml").unwrap_err(),
        "ast-grep rule names.yml changed while it was opened"
    );
}

#[test]
fn symlinked_subfolder_to_outside_is_refused() {
    let tree = Tree::new("symlinked-subfolder");
    fs::create_dir(tree.root.join("elsewhere")).unwrap();
    fs::write(tree.root.join("elsewhere/names.yml"), "id: elsewhere\n").unwrap();
    symlink(tree.root.join("elsewhere"), tree.folder.join("sub")).unwrap();
    assert_eq!(
        open_in_folder(&tree.folder, &tree.folder.join("sub/names.yml"), "sub/names.yml").unwrap_err(),
        "ast-grep rule sub/names.yml changed while it was opened"
    );
}

#[test]
fn plain_file_in_folder_is_opened() {
    let tree = Tree::new("plain-file");
    fs::write(tree.folder.join("names.yml"), "id: names\n").unwrap();
    let mut file = open_in_folder(&tree.folder, &tree.folder.join("names.yml"), "names.yml").unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();
    assert_eq!(text, "id: names\n");
}

#[test]
fn file_in_real_subfolder_is_opened() {
    let tree = Tree::new("real-subfolder");
    fs::create_dir(tree.folder.join("sub")).unwrap();
    fs::write(tree.folder.join("sub/names.yml"), "id: nested\n").unwrap();
    let mut file = open_in_folder(&tree.folder, &tree.folder.join("sub/names.yml"), "sub/names.yml").unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();
    assert_eq!(text, "id: nested\n");
}

#[test]
fn missing_file_reports_read_failure() {
    let tree = Tree::new("missing-file");
    let error = open_in_folder(&tree.folder, &tree.folder.join("missing.yml"), "missing.yml").unwrap_err();
    assert!(error.starts_with("cannot read ast-grep rule missing.yml: "), "{error}");
}

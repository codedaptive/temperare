//! Zone integration tests for opening ast-grep rules without following symlinks.
#![cfg(unix)]

use super::super::open_in_folder::open_in_folder;
use std::fs;
use std::io::Read;
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};

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

#[test]
fn ancestor_swapped_for_a_symlink_is_refused() {
    let tree = Tree::new("ancestor-swap");
    fs::create_dir_all(tree.root.join("a/rules")).unwrap();
    fs::write(tree.root.join("a/rules/names.yml"), "id: names\n").unwrap();
    let folder = fs::canonicalize(tree.root.join("a/rules")).unwrap();
    fs::rename(tree.root.join("a"), tree.root.join("b")).unwrap();
    symlink(tree.root.join("b"), tree.root.join("a")).unwrap();
    assert_eq!(
        open_in_folder(&folder, &folder.join("names.yml"), "names.yml").err().as_deref(),
        Some("ast-grep rule names.yml changed while it was opened")
    );
}

#[test]
fn folder_swapped_for_a_symlink_is_refused() {
    let tree = Tree::new("folder-swap");
    fs::write(tree.folder.join("names.yml"), "id: names\n").unwrap();
    let folder = fs::canonicalize(&tree.folder).unwrap();
    fs::rename(&folder, tree.root.join("real")).unwrap();
    symlink(tree.root.join("real"), &folder).unwrap();
    assert_eq!(
        open_in_folder(&folder, &folder.join("names.yml"), "names.yml").err().as_deref(),
        Some("ast-grep rule names.yml changed while it was opened")
    );
}

fn make_fifo(path: &Path) {
    let mkfifo = procio::Cmd { args: vec![path.as_os_str().to_owned()], ..procio::Cmd::new("mkfifo") };
    let made = procio::run(&mkfifo).unwrap();
    assert_eq!(made.code, Some(0), "{}", String::from_utf8_lossy(&made.stderr));
}

#[test]
fn fifo_held_open_is_refused() {
    let tree = Tree::new("fifo-held");
    let path = tree.folder.join("names.yml");
    make_fifo(&path);
    let _held = fs::OpenOptions::new().read(true).write(true).open(&path).unwrap();
    assert_eq!(
        open_in_folder(&tree.folder, &path, "names.yml").err().as_deref(),
        Some("ast-grep rule names.yml is not a regular file")
    );
}

/// A FIFO opened without O_NONBLOCK and with no writer blocks the open, so this test passes only when the walk
/// never blocks.
#[test]
fn fifo_with_no_other_handle_is_refused_without_blocking() {
    let tree = Tree::new("fifo-unheld");
    let path = tree.folder.join("names.yml");
    make_fifo(&path);
    assert_eq!(
        open_in_folder(&tree.folder, &path, "names.yml").err().as_deref(),
        Some("ast-grep rule names.yml is not a regular file")
    );
}

struct RestorePermissions(PathBuf);

impl Drop for RestorePermissions {
    fn drop(&mut self) {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&self.0, fs::Permissions::from_mode(0o755)).unwrap();
    }
}

#[test]
fn unreadable_ancestor_is_failed_naming_it() {
    use std::os::unix::fs::PermissionsExt;
    let tree = Tree::new("unreadable-ancestor");
    let anc = tree.root.join("anc");
    fs::create_dir_all(anc.join("rules")).unwrap();
    fs::write(anc.join("rules/names.yml"), "id: names\n").unwrap();
    let folder = fs::canonicalize(anc.join("rules")).unwrap();
    let _restore = RestorePermissions(anc.clone());
    fs::set_permissions(&anc, fs::Permissions::from_mode(0o311)).unwrap();
    assert_eq!(
        open_in_folder(&folder, &folder.join("names.yml"), "names.yml").err(),
        Some(format!(
            "cannot read ast-grep rule names.yml: cannot open folder {}: Permission denied (os error 13)",
            anc.display()
        ))
    );
}

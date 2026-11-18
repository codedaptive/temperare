//! ZIT for rule_yaml_path: canonical containment across real files and symlinks.

use super::super::rule_yaml_path::rule_yaml_path;
use std::fs;
use std::path::PathBuf;

struct Tree {
    root: PathBuf,
    folder: PathBuf,
}

impl Tree {
    fn new(test: &str) -> Self {
        let root = std::env::temp_dir().join(format!("engines-zit-{}-rule-yaml-path-{test}", std::process::id()));
        fs::create_dir(&root).unwrap();
        let root = fs::canonicalize(root).unwrap();
        let folder = root.join("rules");
        fs::create_dir(&folder).unwrap();
        fs::write(folder.join("names.yml"), "id: names\n").unwrap();
        fs::write(root.join("outside.yml"), "id: outside\n").unwrap();
        Self { root, folder }
    }
}

impl Drop for Tree {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.root).unwrap();
    }
}

#[test]
fn file_inside_folder_resolves_to_canonical_path() {
    let tree = Tree::new("inside");
    assert_eq!(rule_yaml_path(&tree.folder, "names.yml"), Ok(fs::canonicalize(tree.folder.join("names.yml")).unwrap()));
}

#[test]
fn subdirectory_parent_component_can_resolve_inside_folder() {
    let tree = Tree::new("sub-parent");
    fs::create_dir(tree.folder.join("sub")).unwrap();
    assert_eq!(rule_yaml_path(&tree.folder, "sub/../names.yml"), Ok(fs::canonicalize(tree.folder.join("names.yml")).unwrap()));
}

#[test]
fn parent_file_outside_folder_is_refused() {
    let tree = Tree::new("parent-outside");
    assert_eq!(rule_yaml_path(&tree.folder, "../outside.yml"), Err("ast-grep rule ../outside.yml is outside the rule file's folder".to_string()));
}

#[test]
fn symlink_to_file_outside_folder_is_refused() {
    let tree = Tree::new("symlink-outside");
    std::os::unix::fs::symlink(tree.root.join("outside.yml"), tree.folder.join("link.yml")).unwrap();
    assert_eq!(rule_yaml_path(&tree.folder, "link.yml"), Err("ast-grep rule link.yml is outside the rule file's folder".to_string()));
}

#[test]
fn symlink_to_file_inside_folder_resolves_to_canonical_path() {
    let tree = Tree::new("symlink-inside");
    std::os::unix::fs::symlink(tree.folder.join("names.yml"), tree.folder.join("link.yml")).unwrap();
    assert_eq!(rule_yaml_path(&tree.folder, "link.yml"), Ok(fs::canonicalize(tree.folder.join("names.yml")).unwrap()));
}

#[test]
fn missing_file_reports_read_failure() {
    let tree = Tree::new("missing");
    let err = rule_yaml_path(&tree.folder, "missing.yml").unwrap_err();
    assert!(err.starts_with("cannot read ast-grep rule missing.yml: "), "{err}");
}

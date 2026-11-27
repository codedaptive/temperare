//! Zone integration tests for read_rule_yaml: the checked open across real files, parent paths and hard links.

use super::super::read_rule_yaml::read_rule_yaml;
use std::fs;
use std::path::PathBuf;

struct Tree {
    root: PathBuf,
    folder: PathBuf,
}

impl Tree {
    fn new(test: &str) -> Self {
        let root = std::env::temp_dir().join(format!("engines-zit-{}-read-rule-yaml-{test}", std::process::id()));
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
fn file_inside_folder_is_read() {
    let tree = Tree::new("inside");
    assert_eq!(read_rule_yaml(&tree.folder, "names.yml"), Ok("id: names\n".to_string()));
}

#[test]
fn parent_file_outside_folder_is_refused() {
    let tree = Tree::new("parent-outside");
    assert_eq!(
        read_rule_yaml(&tree.folder, "../outside.yml"),
        Err("ast-grep rule ../outside.yml is outside the rule file's folder".to_string())
    );
}

// `rule_yaml_text` reads the hard-link count on unix only.
#[cfg(unix)]
#[test]
fn hard_link_to_file_outside_folder_is_refused() {
    let tree = Tree::new("hard-link-outside");
    fs::hard_link(tree.root.join("outside.yml"), tree.folder.join("link.yml")).unwrap();
    assert_eq!(
        read_rule_yaml(&tree.folder, "link.yml"),
        Err("ast-grep rule link.yml has more than one hard link: cctao reads a rule YAML only when its folder holds the only link".to_string())
    );
}

#[test]
fn missing_file_reports_read_failure() {
    let tree = Tree::new("missing");
    let err = read_rule_yaml(&tree.folder, "missing.yml").unwrap_err();
    assert!(err.starts_with("cannot read ast-grep rule missing.yml"), "{err}");
}

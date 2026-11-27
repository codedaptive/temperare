//! ZIT for rule_yaml_text: an opened rule YAML checked by its own link count on real files (Unix).
#![cfg(unix)]

use super::super::rule_yaml_text::rule_yaml_text;
use std::fs::{self, File};
use std::path::PathBuf;

struct Tree {
    root: PathBuf,
    folder: PathBuf,
}

impl Tree {
    fn new(test: &str) -> Self {
        let root = std::env::temp_dir().join(format!("engines-zit-{}-rule-yaml-text-{test}", std::process::id()));
        fs::create_dir(&root).unwrap();
        let root = fs::canonicalize(root).unwrap();
        let folder = root.join("rules");
        fs::create_dir(&folder).unwrap();
        fs::write(folder.join("names.yml"), "id: names\n").unwrap();
        fs::write(folder.join("other.yml"), "id: other\n").unwrap();
        Self { root, folder }
    }

    fn open_names(&self) -> File {
        File::open(self.folder.join("names.yml")).unwrap()
    }
}

impl Drop for Tree {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.root).unwrap();
    }
}

#[test]
fn path_swapped_after_open_is_refused() {
    let tree = Tree::new("swapped");
    let file = tree.open_names();
    fs::rename(tree.folder.join("other.yml"), tree.folder.join("names.yml")).unwrap();
    assert_eq!(
        rule_yaml_text(file, "names.yml"),
        Err("ast-grep rule names.yml changed while it was opened".to_string())
    );
}

#[test]
fn path_not_swapped_is_read() {
    let tree = Tree::new("not-swapped");
    let file = tree.open_names();
    assert_eq!(rule_yaml_text(file, "names.yml"), Ok("id: names\n".to_string()));
}

#[test]
fn hard_link_made_after_open_is_refused() {
    let tree = Tree::new("linked-after-open");
    let file = tree.open_names();
    fs::hard_link(tree.folder.join("names.yml"), tree.folder.join("second.yml")).unwrap();
    assert_eq!(
        rule_yaml_text(file, "names.yml"),
        Err("ast-grep rule names.yml has more than one hard link: cctao reads a rule YAML only when its folder holds the only link".to_string())
    );
}

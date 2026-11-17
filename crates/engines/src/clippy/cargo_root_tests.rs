//! ZUT for cargo_root.rs: the nearest folder holding a Cargo.toml, on a temp tree the test makes and removes.

use super::cargo_root;
use std::fs;
use std::path::{Path, PathBuf};

/// A fresh, canonical temp folder for one test.
fn tree(name: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!("engines-zut-{}-cargo-root-{name}", std::process::id()));
    fs::create_dir(&root).unwrap();
    fs::canonicalize(root).unwrap()
}

#[test]
fn the_folder_itself_when_it_holds_a_cargo_toml() {
    let root = tree("itself");
    fs::write(root.join("Cargo.toml"), "").unwrap();
    let got = cargo_root(&root);
    fs::remove_dir_all(&root).unwrap();
    assert_eq!(got, Some(root));
}

#[test]
fn the_nearest_ancestor_wins_over_one_further_up() {
    let root = tree("nearest");
    fs::create_dir_all(root.join("member/src/deep")).unwrap();
    fs::write(root.join("Cargo.toml"), "").unwrap();
    fs::write(root.join("member/Cargo.toml"), "").unwrap();
    let got = cargo_root(&root.join("member/src/deep"));
    fs::remove_dir_all(&root).unwrap();
    assert_eq!(got, Some(root.join("member")));
}

#[test]
fn a_folder_named_cargo_toml_does_not_count() {
    let root = tree("dir-named");
    fs::create_dir_all(root.join("sub/Cargo.toml")).unwrap();
    fs::write(root.join("Cargo.toml"), "").unwrap();
    let got = cargo_root(&root.join("sub"));
    fs::remove_dir_all(&root).unwrap();
    assert_eq!(got, Some(root));
}

#[test]
fn none_when_no_ancestor_holds_one() {
    assert_eq!(cargo_root(Path::new("/engines-zut-no-such-folder/a/b")), None);
}

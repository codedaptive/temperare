//! Zone integration tests for lint_groups_cmd.rs and parse_lint_groups.rs on the real toolchain: `clippy-driver -W help` run in a
//! canonical temp folder the test makes and removes, read into the group list.

use crate::clippy::{lint_groups_cmd, parse_lint_groups};
use std::fs;

#[test]
fn the_installed_toolchain_lists_rustc_and_clippy_groups() {
    let folder = std::env::temp_dir().join(format!("engines-zit-{}-lint-groups", std::process::id()));
    fs::create_dir(&folder).unwrap();
    let folder = fs::canonicalize(folder).unwrap();
    let out = procio::run(&lint_groups_cmd(&folder));
    fs::remove_dir_all(&folder).unwrap();
    let groups = parse_lint_groups(&out.unwrap()).unwrap();
    for name in ["clippy::style", "unused", "warnings"] {
        assert!(groups.iter().any(|group| group == name), "{name} missing from {groups:?}");
    }
}

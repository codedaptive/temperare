//! Unit tests for kit_holds.rs.

use crate::{Kit, RelPath, Scope};

fn kit(paths: &[&str]) -> Kit {
    let paths: Vec<String> = paths.iter().map(|p| p.to_string()).collect();
    let scope = if paths.is_empty() { None } else { Some(Scope::build(&paths, &[], None).unwrap()) };
    Kit { name: "core".to_string(), paths, owner: None, scope }
}

#[test]
fn a_file_under_one_of_its_paths_is_held() {
    let core = kit(&["crates/core/**", "Sources/Core/**"]);
    assert!(core.holds(&RelPath::new("crates/core/src/lib.rs")));
    assert!(core.holds(&RelPath::new("Sources/Core/Model.swift")));
}

#[test]
fn a_file_under_none_of_its_paths_is_not_held() {
    assert!(!kit(&["crates/core/**"]).holds(&RelPath::new("crates/web/src/lib.rs")));
}

#[test]
fn a_kit_with_no_paths_holds_nothing() {
    assert!(!kit(&[]).holds(&RelPath::new("crates/core/src/lib.rs")));
}

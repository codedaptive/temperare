//! ZUT for scope_build.rs.

use crate::Scope;

fn globs(list: &[&str]) -> Vec<String> {
    list.iter().map(|g| g.to_string()).collect()
}

#[test]
fn good_globs_build() {
    let kits = globs(&["kit/**"]);
    assert!(Scope::build(&globs(&["src/**/*.rs"]), &globs(&["target/**"]), Some(&kits)).is_ok());
}

#[test]
fn no_globs_at_all_build() {
    assert!(Scope::build(&[], &[], None).is_ok());
}

#[test]
fn bad_glob_in_paths_is_an_error_naming_it() {
    let err = Scope::build(&globs(&["src/*.rs", "a/["]), &[], None).unwrap_err();
    assert!(err.contains("a/["), "message does not name the glob: {err}");
}

#[test]
fn bad_glob_in_exclude_is_an_error_naming_it() {
    let err = Scope::build(&[], &globs(&["b/{x"]), None).unwrap_err();
    assert!(err.contains("b/{x"), "message does not name the glob: {err}");
}

#[test]
fn bad_glob_in_kit_paths_is_an_error_naming_it() {
    let kits = globs(&["kit/["]);
    let err = Scope::build(&[], &[], Some(&kits)).unwrap_err();
    assert!(err.contains("kit/["), "message does not name the glob: {err}");
}

//! Zone integration tests for engine_hits.rs: an available engine goes to an adapter that reads the disk, given a folder that does not
//! exist.

use crate::engine_hits_tests::{hits_of, not_available};
use crate::types::Stop;

#[test]
fn clippy_goes_to_its_adapter_which_finds_no_cargo_toml() {
    let (got, ran) = hits_of("clippy", "clippy::bool_comparison", true);
    assert_eq!(got, not_available("no Cargo.toml at or above the rule file's folder"));
    assert!(ran.is_empty(), "ran {ran:?}");
}

#[test]
fn ast_grep_goes_to_its_adapter_which_cannot_read_the_rule_yaml() {
    let (got, ran) = hits_of("ast-grep", "rules/x.yml", true);
    let Err(Stop::Failed { code: None, stderr }) = got else { panic!("not Failed with no code: {got:?}") };
    assert!(stderr.starts_with("cannot read ast-grep rule rules/x.yml: "), "{stderr}");
    assert!(ran.is_empty(), "ran {ran:?}");
}

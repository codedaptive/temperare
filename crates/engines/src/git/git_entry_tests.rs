//! Unit tests for git_entry.rs: the fields of `GIT`, its params' names and check, its version read from `git --version`
//! output, its refusal, and its reader that never reads. Nothing runs.

use super::GIT;
use crate::entry::{Params, Pick};
use procio::Output;
use rules::Engine;
use serde_json::{json, Map, Value};
use std::path::Path;

fn out(code: i32, stdout: &[u8]) -> Output {
    Output { code: Some(code), stdout: stdout.to_vec(), stderr: Vec::new() }
}

fn params(value: Value) -> Map<String, Value> {
    let Value::Object(map) = value else { panic!("not an object") };
    map
}

#[test]
fn the_entry_is_git_needing_git_and_passing_nothing() {
    assert_eq!((GIT.name, GIT.engine, GIT.program), ("git", Engine::Git, "git"));
    assert_eq!(GIT.needs, ["git"]);
    assert_eq!(GIT.never, None);
    assert_eq!(GIT.pick, Pick::None);
    assert_eq!(GIT.config, None);
    assert!(GIT.args.is_empty() && GIT.after.is_empty());
}

#[test]
fn the_params_are_its_flows_three_checked_by_read_test_ratio() {
    let Params::Flow { names, check } = GIT.params else { panic!("{:?}", GIT.params) };
    assert_eq!(names, ["max_ratio", "floor", "test_paths"]);
    assert_eq!(check(&params(json!({"max_ratio": 1.5, "floor": 40, "test_paths": ["tests/**"]}))), Ok(()));
    assert!(check(&params(json!({"max_ratio": 0, "floor": 40, "test_paths": ["tests/**"]}))).is_err());
}

#[test]
fn the_version_is_the_word_after_git_version() {
    let probe = GIT.version.unwrap();
    assert_eq!(probe.args, ["--version"]);
    assert_eq!((probe.read)(&out(0, b"git version 2.39.5 (Apple Git-154)\n")), Some("2.39.5".to_string()));
    assert_eq!((probe.read)(&out(0, b"git version 2.47.1\n")), Some("2.47.1".to_string()));
}

#[test]
fn only_test_ratio_is_not_refused_and_git_never_says_a_check_is_unknown() {
    assert_eq!((GIT.refuse)("test_ratio"), None);
    assert!((GIT.refuse)("other").is_some());
    assert_eq!((GIT.unknown)(&out(1, b"test_ratio unknown"), "test_ratio"), None);
}

#[test]
fn the_reader_reads_nothing() {
    assert!((GIT.read)(&out(0, b"diff --git a b\n"), "test_ratio", Path::new("/r"), Path::new("/r")).is_err());
}

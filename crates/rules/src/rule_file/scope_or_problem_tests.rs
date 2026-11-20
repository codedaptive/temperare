//! Unit tests for scope_or_problem.rs. The Err case is a plain Err: no test builds a glob set big enough to hit
//! globset's size limit.

use super::scope_or_problem::scope_or_problem;
use crate::{Problem, RelPath, Scope};

#[test]
fn a_failed_build_is_one_problem_at_the_offset_and_no_scope() {
    // Byte 13 is the 'c' of "core" on line 2, column 2.
    let text = "version = 1\n[core]\n";
    let mut out = Vec::new();
    let scope = scope_or_problem(
        Err("bad glob set: too big".to_string()),
        text,
        13,
        |e| format!("kit \"core\": {e}"),
        "Split this kit's paths into smaller kits.",
        &mut out,
    );
    assert!(scope.is_none());
    assert_eq!(
        out,
        vec![Problem {
            line: 2,
            col: 2,
            what: "kit \"core\": bad glob set: too big".to_string(),
            fix: "Split this kit's paths into smaller kits.".to_string(),
        }]
    );
}

#[test]
fn a_built_scope_comes_back_with_no_problem() {
    let mut out = Vec::new();
    let scope = scope_or_problem(Scope::build(&["src/**".into()], &[], None), "", 0, |e| e, "unused", &mut out)
        .expect("a scope");
    assert_eq!(out, vec![]);
    assert!(scope.matches(&RelPath::new("src/a.rs")));
}

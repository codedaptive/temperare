//! ZUT for write_problems.rs: one problem as text and as the `--json` object, each with exit 2.

use super::write_problems::write_problems;
use rules::Problem;
use std::path::Path;

fn one_problem() -> Vec<Problem> {
    vec![Problem { line: 1, col: 11, what: "version must be 1".to_string(), fix: "Write version = 1.".to_string() }]
}

fn written(json: bool) -> (u8, String) {
    let mut out = Vec::new();
    let code = write_problems(Path::new("/repo/temperare.toml"), &one_problem(), json, &mut out);
    (code, String::from_utf8(out).unwrap())
}

#[test]
fn text_is_each_problem_line_then_the_next_line() {
    assert_eq!(
        written(false),
        (
            2,
            "/repo/temperare.toml:1:11  version must be 1  Write version = 1.\n\
             next: fix each problem listed for /repo/temperare.toml\n"
                .to_string()
        )
    );
}

#[test]
fn json_is_the_problems_object_with_exit_and_next_on_one_line() {
    assert_eq!(
        written(true),
        (
            2,
            "{\"exit\":2,\"next\":\"fix each problem listed for /repo/temperare.toml\",\
             \"problems\":[{\"col\":11,\"fix\":\"Write version = 1.\",\"line\":1,\"what\":\"version must be 1\"}],\
             \"rule_file\":\"/repo/temperare.toml\"}\n"
                .to_string()
        )
    );
}

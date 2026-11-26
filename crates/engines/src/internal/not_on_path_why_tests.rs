//! ZUT for not_on_path_why.rs: the why names the program as given.

use crate::not_on_path_why::not_on_path_why;

#[test]
fn the_why_names_the_missing_program() {
    let cases = [("ruff", "ruff not found on PATH"), ("clippy-driver", "clippy-driver not found on PATH")];
    for (program, want) in cases {
        assert_eq!(not_on_path_why(program), want, "{program}");
    }
}

//! ZUT for syntax_problem.rs.

use super::syntax_problem::syntax_problem;
use toml::de::DeTable;

#[test]
fn a_syntax_error_is_a_problem_at_its_span_with_the_toml_message() {
    // The second `=` of line 2 is the first byte toml cannot read: line 2, column 6.
    let text = "version = 1\nid = = 3\n";
    let e = DeTable::parse(text).expect_err("two `=` in a row");
    let problem = syntax_problem(text, &e);
    assert_eq!((problem.line, problem.col), (2, 6), "{problem:?}");
    assert_eq!(problem.what, format!("TOML syntax error: {}", e.message().trim()));
    assert!(problem.what.len() > "TOML syntax error: ".len(), "toml's message is kept: {problem:?}");
    assert_eq!(problem.fix, "Correct the TOML syntax at this position.");
}

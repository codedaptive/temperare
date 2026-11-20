//! ZUT for ast_grep_parse_error.rs: the error's form, with the exit code and the quoted stderr.

use procio::Output;

use super::ast_grep_parse_error;

#[test]
fn the_error_names_the_exit_and_quotes_stderr() {
    let out = Output { code: Some(8), stdout: b"ignored".to_vec(), stderr: b"Error: Cannot parse rule".to_vec() };
    let why = ast_grep_parse_error(&out).why;
    assert_eq!(why, "ast-grep output is not JSON match lines (exit Some(8)): Error: Cannot parse rule");
}

//! `ast_grep_parse_error`: the error for ast-grep output that is not an answer.

use procio::Output;

use crate::printed_start::printed_start;
use crate::ParseError;

/// The error for ast-grep output that is not JSON match lines, quoting the first 200 bytes of stderr, or of stdout
/// when stderr is empty. Both readers of ast-grep's JSON (hits and count matches) give this one form.
pub(crate) fn ast_grep_parse_error(out: &Output) -> ParseError {
    let quoted = printed_start(out);
    ParseError { why: format!("ast-grep output is not JSON match lines (exit {:?}): {quoted}", out.code) }
}

//! `parse_count_matches`: `ast-grep scan --json=stream` output read into count matches.

use procio::Output;
use serde::Deserialize;

use super::{ast_grep_parse_error, CountMatch};
use crate::ParseError;

/// One match line as ast-grep prints it; fields not read here are ignored.
#[derive(Deserialize)]
struct Match {
    file: String,
    text: String,
    range: Range,
}

#[derive(Deserialize)]
struct Range {
    #[serde(rename = "byteOffset")]
    byte_offset: ByteOffset,
    start: Position,
    end: Position,
}

#[derive(Deserialize)]
struct ByteOffset {
    start: usize,
    end: usize,
}

#[derive(Deserialize)]
struct Position {
    line: u32,
    column: u32,
}

/// The matches in ast-grep's JSON lines, in output order, under the same exit rules as `parse_ast_grep`: empty
/// stdout is no matches when ast-grep exits 0 and not an answer otherwise; output that is not UTF-8 or a line that is
/// not a match is not an answer.
pub(crate) fn parse_count_matches(out: &Output) -> Result<Vec<CountMatch>, ParseError> {
    let stdout = std::str::from_utf8(&out.stdout).map_err(|_| ast_grep_parse_error(out))?;
    if stdout.trim().is_empty() && out.code != Some(0) {
        return Err(ast_grep_parse_error(out));
    }
    stdout
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let found: Match = serde_json::from_str(line).map_err(|_| ast_grep_parse_error(out))?;
            Ok(CountMatch {
                file: found.file,
                text: found.text,
                start_byte: found.range.byte_offset.start,
                end_byte: found.range.byte_offset.end,
                start_line: found.range.start.line,
                end_line: found.range.end.line,
                end_column: found.range.end.column,
            })
        })
        .collect()
}

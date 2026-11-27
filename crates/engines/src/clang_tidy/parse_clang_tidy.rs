//! `parse_clang_tidy`: clang-tidy's text output, read into the hits of one rule.

use crate::printed_start::printed_start;
use crate::rel_to_folder::rel_to_folder;
use crate::types::{Hit, ParseError};
use procio::Output;
use std::path::Path;

/// One diagnostic header line: `<path>:<line>:<col>: <level>: <message> [<names>]`.
struct Header<'a> {
    path: &'a str,
    line: u32,
    col: u32,
    message: &'a str,
    names: &'a str,
}

/// The hits of rule `check` in `out`, in output order. Any exit but 0 is a `ParseError` quoting the first stdout line
/// holding `: error: `, trimmed (a compile error exits 1 and prints it there, while stderr may hold only
/// `[1/2] Processing file ...` progress lines), or `printed_start`'s quote when stdout has no such line. On exit 0 each stdout line `<path>:<line>:<col>: warning: <message> [<names>]`,
/// or with `error:` for `warning:`, is a hit when one of its comma-separated `<names>` is `check` exactly; every other
/// line (source snippets, notes, counts) is ignored. `cwd` is where clang-tidy ran and `folder` the rule file's folder,
/// both canonical; a hit in a file outside `folder` is skipped. A kept line with a relative path is a `ParseError`:
/// clang-tidy prints a path relative to its compile database's `directory` when the database's `file` entry is
/// relative, whatever its own working directory, so cctao cannot tell where the file is.
pub fn parse_clang_tidy(out: &Output, check: &str, cwd: &Path, folder: &Path) -> Result<Vec<Hit>, ParseError> {
    let stdout = String::from_utf8_lossy(&out.stdout);
    if out.code != Some(0) {
        let quote = stdout.lines().find(|line| line.contains(": error: ")).map(str::trim);
        return Err(ParseError {
            why: format!("clang-tidy gave no findings: {}", quote.map_or_else(|| printed_start(out), str::to_string)),
        });
    }
    let mut hits = Vec::new();
    for header in stdout.lines().filter_map(header) {
        if !header.names.split(',').any(|name| name == check) {
            continue;
        }
        if !Path::new(header.path).is_absolute() {
            return Err(ParseError {
                why: format!(
                    "clang-tidy printed `{}` relative to its compile database's directory; cctao reads its findings only when the database's `file` entries are absolute paths",
                    header.path
                ),
            });
        }
        if let Some(path) = rel_to_folder(cwd, folder, header.path) {
            hits.push(Hit { path, line: Some(header.line), col: Some(header.col), what: header.message.to_string() });
        }
    }
    Ok(hits)
}

/// `line` read as a diagnostic header, or None when it is not one: no `: warning: ` or `: error: ` marker, a line or
/// column that is not a number, or no `[...]` tag at its end.
fn header(line: &str) -> Option<Header<'_>> {
    let (left, right) = line.split_once(": warning: ").or_else(|| line.split_once(": error: "))?;
    let mut place = left.rsplitn(3, ':');
    let col = place.next()?.parse().ok()?;
    let line_number = place.next()?.parse().ok()?;
    let path = place.next()?;
    let (message, names) = right.strip_suffix(']')?.rsplit_once('[')?;
    Some(Header { path, line: line_number, col, message: message.trim_end(), names })
}

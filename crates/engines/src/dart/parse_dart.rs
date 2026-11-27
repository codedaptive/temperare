//! `parse_dart`: `dart analyze --format machine` output, read into the hits of one rule.

use crate::printed_start::printed_start;
use crate::rel_to_folder::rel_to_folder;
use crate::types::{Hit, ParseError};
use procio::Output;
use std::path::Path;

/// The hits of rule `check` in `out`, in output order. Each non-empty stdout line is one diagnostic,
/// `SEVERITY|TYPE|CODE|PATH|LINE|COLUMN|LENGTH|MESSAGE`, split on the `|`s no backslash escapes, each field then
/// unescaped (`\|` is `|`, `\\` is `\`; dart escapes the path and the message so). A line whose CODE is `check`, case
/// aside (dart prints codes upper case, rules name them lower case), is a hit: its path, absolute or relative to
/// `cwd`, made relative to `folder` (a file outside `folder` is not a hit), its line and column, and the message as
/// its `what`. Exit 0, 2 or 3 is an answer, the worst severity found; the exit codes measured (fixtures/commands.txt)
/// are 0 for nothing or infos only, 2 for warnings, 3 for errors. Any other exit (1 included: cctao passes no
/// `--fatal-infos`, and it was never measured), or none, is a `ParseError` quoting what dart printed. So is a line
/// that is not 8 fields, or whose line or column is not a number, whatever the check.
pub(crate) fn parse_dart(out: &Output, check: &str, cwd: &Path, folder: &Path) -> Result<Vec<Hit>, ParseError> {
    if !matches!(out.code, Some(0 | 2 | 3)) {
        let exit = match out.code {
            Some(code) => format!("exited {code}"),
            None => "was stopped by a signal".to_string(),
        };
        return Err(failed(format!("dart analyze {exit}, not 0, 2 or 3: {}", printed_start(out))));
    }
    let mut hits = Vec::new();
    for line in String::from_utf8_lossy(&out.stdout).lines().filter(|line| !line.is_empty()) {
        let not_a_diagnostic = || failed(format!("dart analyze printed a line that is not a diagnostic: {line}"));
        let fields = fields_of(line);
        let [_, _, code, path, number, col, _, what] = &fields[..] else {
            return Err(not_a_diagnostic());
        };
        let (Ok(number), Ok(col)) = (number.parse(), col.parse()) else {
            return Err(not_a_diagnostic());
        };
        if !code.eq_ignore_ascii_case(check) {
            continue;
        }
        if let Some(path) = rel_to_folder(cwd, folder, path) {
            hits.push(Hit { path, line: Some(number), col: Some(col), what: what.clone() });
        }
    }
    Ok(hits)
}

/// `line` split on its unescaped `|`s, each field unescaped: a backslash keeps the character after it as it is.
fn fields_of(line: &str) -> Vec<String> {
    let mut fields = Vec::new();
    let mut field = String::new();
    let mut chars = line.chars();
    while let Some(c) = chars.next() {
        match c {
            '\\' => field.push(chars.next().unwrap_or('\\')),
            '|' => fields.push(std::mem::take(&mut field)),
            _ => field.push(c),
        }
    }
    fields.push(field);
    fields
}

fn failed(why: String) -> ParseError {
    ParseError { why }
}

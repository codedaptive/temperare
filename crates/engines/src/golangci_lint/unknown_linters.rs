//! `unknown_linters`: golangci-lint's words saying a linter of the rule's check is not one it knows.

use procio::Output;

/// The first line of `out`'s stderr that contains "unknown linters: '<list>'" where some comma item of `<list>` is
/// one of `check`'s comma parts, trimmed; `None` when no line does. golangci-lint 2.15.0 given an unknown linter in
/// `--enable-only` exits 3 with empty stdout and prints "Error: unknown linters: 'nosuchlinter', run 'golangci-lint
/// help linters' to see the list of supported linters" (two unknown: `'foo,bar'`). A line naming only other linters
/// is not about the check, so it is left to the reader.
pub(crate) fn unknown_linters(out: &Output, check: &str) -> Option<String> {
    String::from_utf8_lossy(&out.stderr)
        .lines()
        .find(|line| names_a_part(line, check))
        .map(|line| line.trim().to_string())
}

/// Whether `line` holds "unknown linters: '<list>'" with an item of `<list>` among `check`'s comma parts.
fn names_a_part(line: &str, check: &str) -> bool {
    let Some((_, after)) = line.split_once("unknown linters: '") else {
        return false;
    };
    let Some((list, _)) = after.split_once('\'') else {
        return false;
    };
    list.split(',').any(|linter| check.split(',').any(|part| part == linter))
}

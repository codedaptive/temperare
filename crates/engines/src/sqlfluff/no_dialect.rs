//! `no_dialect`: sqlfluff refusing to lint because the repository sets no SQL dialect.

use procio::Output;

/// sqlfluff 4.5.0's words, in its "User Error: No dialect was specified. ..." line (exit 2, no JSON).
const NO_DIALECT: &str = "No dialect was specified";

/// Why the rule is not available: tm passes no `--dialect`, since which SQL a repository writes is the repository's
/// own setting.
const WHY: &str =
    "no dialect is set in the repository's sqlfluff config: set `dialect` under `[sqlfluff]` in its .sqlfluff";

/// `WHY` when `out`'s stderr holds `NO_DIALECT`; None otherwise.
pub(crate) fn no_dialect(out: &Output) -> Option<String> {
    String::from_utf8_lossy(&out.stderr).contains(NO_DIALECT).then(|| WHY.to_string())
}

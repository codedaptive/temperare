//! `check_duplicate_case_file`: a case lists each of its files once.

use super::problem_at::problem_at;
use crate::Problem;

/// Whether the case file name `file`, listed at byte `file_at` of the set's `text` in the `kind` case of baseline rule
/// `id`, is not among `earlier`, the names that case lists before it. A repeat is a name that is the same as an earlier
/// one once both are normalised (see `normalised`), so `./a.rs` repeats `a.rs`. A repeat is a Problem at its entry,
/// named as written, and false: the first entry alone stands for the file.
pub(crate) fn check_duplicate_case_file(
    text: &str,
    file: &str,
    file_at: usize,
    earlier: &[String],
    id: &str,
    kind: &str,
    out: &mut Vec<Problem>,
) -> bool {
    let repeat = earlier.iter().any(|name| normalised(name) == normalised(file));
    if repeat {
        out.push(problem_at(
            text,
            file_at,
            format!("case file \"{file}\" is listed twice in the {kind} case of baseline rule \"{id}\""),
            "Remove the repeated entry from files.",
        ));
    }
    !repeat
}

/// A case file name as a root marker and its parts: `/` and `\` both separate, and `.` and empty parts are dropped, so
/// `./src//a.rs` and `src\a.rs` match `src/a.rs`. The root marker keeps `/a.rs` apart from `a.rs`, so `/a.rs` after
/// `a.rs` is not a repeat and gets the name check's own Problem.
fn normalised(name: &str) -> (bool, Vec<&str>) {
    (
        name.starts_with(['/', '\\']),
        name.split(['/', '\\']).filter(|part| !part.is_empty() && *part != ".").collect(),
    )
}

//! `check_case_file_name`: a case file name stays inside its case folder.

use super::problem_at::problem_at;
use crate::Problem;

/// Whether the case file name `file`, listed at byte `file_at` of the set's `text` in the `kind` case of baseline rule
/// `id`, stays inside `cases/<id>/<kind>/`. The name is read as text, the same on every host, with `/` and `\` both
/// separating its parts. A name that starts with a separator (a root), starts with a drive letter (an ASCII letter
/// then `:`), or has a part that is exactly `..` could name a file outside the case folder: it is a Problem at its
/// entry, and false. A `:` anywhere else, as in `ab:c.rs`, is not a drive.
pub(crate) fn check_case_file_name(
    text: &str,
    file: &str,
    file_at: usize,
    id: &str,
    kind: &str,
    out: &mut Vec<Problem>,
) -> bool {
    let drive = matches!(file.as_bytes(), [letter, b':', ..] if letter.is_ascii_alphabetic());
    let escapes = drive || file.starts_with(['/', '\\']) || file.split(['/', '\\']).any(|part| part == "..");
    if escapes {
        out.push(problem_at(
            text,
            file_at,
            format!("case file \"{file}\" of baseline rule \"{id}\" must be a relative path inside its case folder"),
            &format!("Name the file relative to cases/{id}/{kind}/, with no leading / or \\, no drive letter and no .. part."),
        ));
    }
    !escapes
}

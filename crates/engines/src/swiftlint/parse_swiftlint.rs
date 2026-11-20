//! `parse_swiftlint`: swiftlint's JSON reporter output, read into the hits of one rule.

use super::unknown_rule;
use crate::printed_start::printed_start;
use crate::rel_to_folder::rel_to_folder;
use crate::types::{Hit, ParseError};
use procio::Output;
use serde::Deserialize;
use rules::RelPath;
use std::path::{Path, PathBuf};

/// One violation as swiftlint's JSON reporter prints it; other fields (severity, type) are not read.
#[derive(Deserialize)]
struct Violation {
    file: String,
    line: u32,
    /// Null for a violation of a whole line or file (trailing_newline, for one).
    character: Option<u32>,
    reason: String,
    rule_id: String,
}

/// The hits of rule `check` in `out`, in output order. stdout is one JSON array of violations; any exit code with
/// such stdout is an answer (swiftlint exits 2 when a violation is an error). `cwd` is where swiftlint ran and
/// `folder` the rule file's folder, both canonical. A violation's path is placed under `folder` as printed or, when
/// that fails, with a leading `/private` removed from `cwd` and `folder`: on macOS swiftlint prints `/tmp/...` and
/// `/var/...` for files under the canonical `/private/tmp/...` and `/private/var/...`. A checked violation whose path
/// is under `folder` in neither form is a `ParseError` naming that path, never a dropped finding (S-35).
/// Every violation swiftlint reports for one valid `--only-rule` id carries that id, so keeping `rule_id == check`
/// drops nothing for the checks that reach here: a `custom_rules` check, whose violations carry the custom rule's own
/// id, is refused before running by `swiftlint_refusal`. swiftlint given an id it does not know warns on stderr that
/// it "is not a valid rule identifier", prints `[]` and exits 0; that is a `ParseError` naming `check` and quoting
/// swiftlint's warning line, never a clean answer (`run_plain` stops such a run as an unknown check before it reads
/// it, through the entry's `unknown_rule`). stdout that is not the array is a `ParseError` too, quoting the
/// first 200 bytes of stderr, or of stdout when stderr is empty.
pub fn parse_swiftlint(out: &Output, check: &str, cwd: &Path, folder: &Path) -> Result<Vec<Hit>, ParseError> {
    if let Some(line) = unknown_rule(out, check) {
        return Err(ParseError {
            why: format!(
                "swiftlint check `{check}` is not a rule this swiftlint knows ({line}): tm cannot report it; name a \
                 rule `swiftlint rules` lists, or a custom rule the rule's config defines"
            ),
        });
    }
    let violations: Vec<Violation> = serde_json::from_slice(&out.stdout).map_err(|_| not_findings(out))?;
    violations
        .into_iter()
        .filter(|violation| violation.rule_id == check)
        .map(|violation| {
            Ok(Hit {
                path: place(cwd, folder, &violation.file)?,
                line: Some(violation.line),
                col: violation.character,
                what: violation.reason,
            })
        })
        .collect()
}

/// `file` relative to `folder`, as printed or with `/private` removed from `cwd` and `folder`; else the error naming it.
fn place(cwd: &Path, folder: &Path, file: &str) -> Result<RelPath, ParseError> {
    rel_to_folder(cwd, folder, file)
        .or_else(|| rel_to_folder(&without_private(cwd), &without_private(folder), file))
        .ok_or_else(|| ParseError {
            why: format!("swiftlint printed `{file}`, a path not under the rule folder `{}`", folder.display()),
        })
}

/// `path` with a leading `/private` component removed and re-rooted at `/`; `path` itself when it has none.
fn without_private(path: &Path) -> PathBuf {
    path.strip_prefix("/private").map_or_else(|_| path.to_path_buf(), |rest| Path::new("/").join(rest))
}

/// The error for output that is not swiftlint's JSON: it names swiftlint and quotes what it printed.
fn not_findings(out: &Output) -> ParseError {
    ParseError { why: format!("swiftlint printed no JSON findings: {}", printed_start(out)) }
}

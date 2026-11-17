//! `write_problems`: a rule file's problems as a subcommand's exit 2, as text or as one JSON object.

use crate::{next_step, problem_line, problems_json};
use serde_json::json;
use std::io::Write;
use std::path::Path;

/// Writes the problems of the rule file at `path` on `out`: as text, one `problem_line` each then
/// `next: fix each problem listed for <path>`; under `json`, the `problems_json` object with `exit` 2 and that `next`,
/// on one line. Returns 2. Write errors are ignored: a closed pipe is not tm's failure.
pub(crate) fn write_problems(path: &Path, problems: &[rules::Problem], json: bool, out: &mut dyn Write) -> u8 {
    let fix = format!("fix each problem listed for {}", path.display());
    let next = next_step(2, Some(&fix), path, &[]);
    if json {
        let mut value = problems_json(path, problems);
        if let Some(object) = value.as_object_mut() {
            object.insert("exit".to_string(), json!(2));
            object.insert("next".to_string(), json!(next));
        }
        let _ = writeln!(out, "{value}");
    } else {
        let lines: String = problems.iter().map(|p| problem_line(path, p) + "\n").collect();
        let _ = writeln!(out, "{lines}next: {next}");
    }
    2
}

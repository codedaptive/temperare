//! `problem_line`: one rule-file problem as one line of text.

use std::path::Path;

/// `<file>:<line>:<col>  <what>  <fix>`, two spaces between the parts and no newline.
pub fn problem_line(file: &Path, p: &rules::Problem) -> String {
    format!("{}:{}:{}  {}  {}", file.display(), p.line, p.col, p.what, p.fix)
}

//! `problems_json`: a rule file's problems as one JSON object.

use serde_json::{json, Value};
use std::path::Path;

/// `{"rule_file": <file>, "problems": [{"line", "col", "what", "fix"}]}`, problems in the order given.
pub fn problems_json(file: &Path, problems: &[rules::Problem]) -> Value {
    let problems: Vec<Value> =
        problems.iter().map(|p| json!({"line": p.line, "col": p.col, "what": p.what, "fix": p.fix})).collect();
    json!({"rule_file": file.display().to_string(), "problems": problems})
}

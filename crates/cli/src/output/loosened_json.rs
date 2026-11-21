//! `loosened_json`: what `tm rules --loosened` found, as one JSON object.

use serde_json::{json, Value};
use std::path::Path;

/// `{"rule_file", "base", "loosened", "count"}`: the rule file's path, the base revision as given, each loosening in
/// `list`'s order as `{"kind", "about", "before", "after"}` (the kind by its serde name; `before` and `after` present
/// only when the loosening has them), and how many there are.
pub fn loosened_json(file: &Path, base: &str, list: &[rules::Loosening]) -> Value {
    json!({
        "rule_file": file.display().to_string(),
        "base": base,
        "loosened": list,
        "count": list.len(),
    })
}

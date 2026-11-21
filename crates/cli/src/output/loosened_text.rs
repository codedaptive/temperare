//! `loosened_text`: what `tm rules --loosened` found, as text.

use super::serde_name::serde_name;
use std::fmt::Write;
use std::path::Path;

/// The rule file's path, the base revision as given, the count, then one line per loosening in `list`'s order:
/// `  <kind>  <about>`, then `  before: <before>` and `  after: <after>` each only when the loosening has it. The kind
/// takes its serde name, the report's name. Every line ends in a newline.
pub fn loosened_text(file: &Path, base: &str, list: &[rules::Loosening]) -> String {
    let mut text = format!("rule file: {}\nbase: {base}\nloosened ({}):\n", file.display(), list.len());
    for l in list {
        // Writing to a String cannot fail.
        let _ = write!(text, "  {}  {}", serde_name(serde_json::to_value(l.kind)), l.about);
        if let Some(before) = &l.before {
            let _ = write!(text, "  before: {before}");
        }
        if let Some(after) = &l.after {
            let _ = write!(text, "  after: {after}");
        }
        text.push('\n');
    }
    text
}

//! `not_available_names`: the engines a report could not run, for the exit-3 next line.

use super::serde_name::serde_name;
use report::Report;

/// The rule-file names (`clippy`, `swiftlint`, `ruff`, `ast-grep`, `tessera`) of the engines in
/// `report.not_available`, each once, in first-seen order.
pub fn not_available_names(report: &Report) -> Vec<String> {
    let mut names: Vec<String> = Vec::new();
    for n in &report.not_available {
        let name = serde_name(serde_json::to_value(n.engine));
        if !names.contains(&name) {
            names.push(name);
        }
    }
    names
}

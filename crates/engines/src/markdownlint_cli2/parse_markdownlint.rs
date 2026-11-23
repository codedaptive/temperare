//! `parse_markdownlint`: markdownlint-cli2's JSON output, as tm's options module prints it, read into the hits of one
//! rule.

use crate::printed_start::printed_start;
use crate::rel_to_folder::rel_to_folder;
use crate::types::{Hit, ParseError};
use procio::Output;
use serde::Deserialize;
use std::path::Path;

/// One result of markdownlint's `results` array; `ruleInformation`, `fixInfo` and `severity` are not read.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct LintError {
    /// Relative to the folder markdownlint-cli2 ran in.
    file_name: String,
    line_number: u32,
    /// The rule's id, then its aliases: `["MD013", "line-length"]`.
    rule_names: Vec<String>,
    rule_description: String,
    error_detail: Option<String>,
    error_context: Option<String>,
    /// `[column, length]`, both from 1; null when the rule marks no range.
    error_range: Option<Vec<u32>>,
}

/// The hits of rule `check` in `out`, in output order: each result one of whose `ruleNames` equals `check`, ignoring
/// ASCII case, as markdownlint matches names. A hit's path is its `fileName` made relative to `folder` from `cwd`
/// (both canonical; a file outside `folder` gives no hit), its line the `lineNumber`, its column the first number of
/// `errorRange` (None when that is null), and its text the default formatter's wording: the names joined by `/`, a
/// space and the description, then ` [<errorDetail>]` and ` [Context: "<errorContext>"]` for those that are neither
/// null nor empty.
///
/// markdownlint-cli2 exits 0 when clean, 1 with findings and 2 for a usage or config error, so the exit code is not
/// read: stdout that is not one JSON array of results (a config error prints its banner and no array) is a
/// `ParseError` naming markdownlint-cli2 and quoting `printed_start`.
pub fn parse_markdownlint(out: &Output, check: &str, cwd: &Path, folder: &Path) -> Result<Vec<Hit>, ParseError> {
    let results: Vec<LintError> = serde_json::from_slice(&out.stdout).map_err(|_| ParseError {
        why: format!("markdownlint-cli2 printed no JSON results: {}", printed_start(out)),
    })?;
    Ok(results
        .into_iter()
        .filter(|result| result.rule_names.iter().any(|name| name.eq_ignore_ascii_case(check)))
        .filter_map(|result| {
            let path = rel_to_folder(cwd, folder, &result.file_name)?;
            let mut what = format!("{} {}", result.rule_names.join("/"), result.rule_description);
            if let Some(detail) = result.error_detail.as_deref().filter(|detail| !detail.is_empty()) {
                what.push_str(&format!(" [{detail}]"));
            }
            if let Some(context) = result.error_context.as_deref().filter(|context| !context.is_empty()) {
                what.push_str(&format!(" [Context: \"{context}\"]"));
            }
            let col = result.error_range.as_ref().and_then(|range| range.first().copied());
            Some(Hit { path, line: Some(result.line_number), col, what })
        })
        .collect())
}

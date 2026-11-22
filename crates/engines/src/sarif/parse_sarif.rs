//! `parse_sarif`: a SARIF 2.1 log, read into the hits of one rule.

use super::sarif_path::sarif_path;
use crate::printed_start::printed_start;
use crate::rel_to_folder::rel_to_folder;
use crate::types::{Hit, ParseError};
use procio::Output;
use serde::Deserialize;
use std::path::Path;

/// A SARIF log: only its runs are read.
#[derive(Deserialize)]
struct Log {
    runs: Vec<Run>,
}

/// One run of a tool: only its results are read; a run with no `results` has none.
#[derive(Deserialize)]
struct Run {
    #[serde(default)]
    results: Vec<SarifResult>,
}

/// One result: its rule id (`ruleId`, else `rule.id`), message text and locations, of which the first is read.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SarifResult {
    rule_id: Option<String>,
    rule: Option<RuleReference>,
    message: Option<Message>,
    #[serde(default)]
    locations: Vec<Location>,
}

#[derive(Deserialize)]
struct RuleReference {
    id: Option<String>,
}

#[derive(Deserialize)]
struct Message {
    text: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Location {
    physical_location: Option<PhysicalLocation>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct PhysicalLocation {
    artifact_location: Option<ArtifactLocation>,
    region: Option<Region>,
}

/// The file a location names; any `uriBaseId` is not read, so a relative `uri` is taken as relative to `cwd`.
#[derive(Deserialize)]
struct ArtifactLocation {
    uri: Option<String>,
}

/// Where a result starts: 1-based line and column, as SARIF counts them.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Region {
    start_line: Option<u32>,
    start_column: Option<u32>,
}

/// The hits of rule `check` in `out`, runs in order and each run's results in order. stdout is one SARIF 2.1 log;
/// the fields read are `runs[].results[]`, and of each result `ruleId` (else `rule.id`), `message.text` (empty when
/// absent) and its first location's `physicalLocation.artifactLocation.uri` and `region.startLine`/`startColumn`
/// (line and column None when there is no region). A result is kept when its rule id equals one comma-separated part
/// of `check`, or `check` is `ALL`; one with no rule id never is. The URI is read by `sarif_path` and made relative
/// to `folder` with `cwd` as the engine's working directory; a result with no location, or in a file outside
/// `folder`, is not a hit. Any exit code with a SARIF log on stdout is an answer; stdout that is not one (not JSON,
/// or no `runs` array) is a `ParseError` quoting the start of what the engine printed. An entry whose
/// engine prints SARIF sets its reader to `parse_sarif`, or to a reader that calls it (`parse_gitleaks` does).
pub fn parse_sarif(out: &Output, check: &str, cwd: &Path, folder: &Path) -> Result<Vec<Hit>, ParseError> {
    let log: Log = serde_json::from_slice(&out.stdout)
        .map_err(|_| ParseError { why: format!("SARIF output is not a SARIF log: {}", printed_start(out)) })?;
    Ok(log
        .runs
        .into_iter()
        .flat_map(|run| run.results)
        .filter(|result| rule_id(result).is_some_and(|id| check == "ALL" || check.split(',').any(|part| part == id)))
        .filter_map(|result| hit(result, cwd, folder))
        .collect())
}

/// The result's rule id: `ruleId`, else `rule.id`.
fn rule_id(result: &SarifResult) -> Option<&str> {
    result.rule_id.as_deref().or_else(|| result.rule.as_ref()?.id.as_deref())
}

/// The hit for `result`; None when it has no location or its file is outside `folder`.
fn hit(result: SarifResult, cwd: &Path, folder: &Path) -> Option<Hit> {
    let physical = result.locations.into_iter().next()?.physical_location?;
    let uri = physical.artifact_location?.uri?;
    Some(Hit {
        path: rel_to_folder(cwd, folder, &sarif_path(&uri))?,
        line: physical.region.as_ref().and_then(|region| region.start_line),
        col: physical.region.as_ref().and_then(|region| region.start_column),
        what: result.message.and_then(|message| message.text).unwrap_or_default(),
    })
}

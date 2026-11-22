//! `parse_gitleaks`: gitleaks' SARIF log, read into the hits of one rule without the secret.

use crate::sarif::parse_sarif;
use crate::types::{Hit, ParseError};
use procio::Output;
use std::path::Path;

/// Every gitleaks hit's text. gitleaks' own message holds only the rule id and the file, which the rule and the hit's
/// path already give; a fixed text means no redaction tm does not control (a repo config, a later gitleaks) can let
/// the secret through.
const WHAT: &str = "gitleaks found a secret here; tm does not print it";

/// Why stdout that is not a SARIF log is no answer. It quotes nothing: stdout is where gitleaks puts findings.
const NO_LOG: &str = "gitleaks printed no SARIF log and no error";

/// `parse_sarif`'s hits of `check` in `out`, each with its `what` replaced by one fixed text, so no message text
/// gitleaks printed reaches a hit (the path, line and column are still gitleaks'). When stdout is not a SARIF log the
/// error is "gitleaks printed no SARIF log and no error", never `parse_sarif`'s, which quotes stdout; `parse_failed`
/// shows the run's stderr in its place when there is any.
pub fn parse_gitleaks(out: &Output, check: &str, cwd: &Path, folder: &Path) -> Result<Vec<Hit>, ParseError> {
    let hits = parse_sarif(out, check, cwd, folder).map_err(|_| ParseError { why: NO_LOG.to_string() })?;
    Ok(hits.into_iter().map(|hit| Hit { what: WHAT.to_string(), ..hit }).collect())
}

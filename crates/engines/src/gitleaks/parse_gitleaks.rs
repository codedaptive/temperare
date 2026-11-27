//! `parse_gitleaks`: gitleaks' SARIF log, read into the hits of one rule without the secret.

use crate::sarif::{read_sarif, SarifFailure};
use crate::types::{Hit, ParseError};
use procio::Output;
use std::path::Path;

/// Every gitleaks hit's text. gitleaks' own message holds only the rule id and the file, which the rule and the hit's
/// path already give; a fixed text means no redaction cctao does not control (a repo config, a later gitleaks) can let
/// the secret through.
const WHAT: &str = "gitleaks found a secret here; cctao does not print it";

/// The error when stdout is not a SARIF log, in place of `read_sarif`'s, which quotes stdout when stderr is empty:
/// stdout is where gitleaks puts findings, so this quotes nothing.
const NO_LOG: &str = "gitleaks printed no SARIF log and no error";

/// `read_sarif`'s hits of `check` in `out`, each with its `what` replaced by one fixed text, so no message text
/// gitleaks printed reaches a hit (the path, line and column are still gitleaks'). When stdout is not a SARIF log the
/// error is "gitleaks printed no SARIF log and no error"; when a checked result names a path not under `folder` it is
/// `read_sarif`'s own, which quotes only that URI and the folder. `parse_failed` shows the run's stderr in place of
/// either when there is any.
pub fn parse_gitleaks(out: &Output, check: &str, cwd: &Path, folder: &Path) -> Result<Vec<Hit>, ParseError> {
    let hits = read_sarif(out, check, cwd, folder).map_err(|failure| match failure {
        SarifFailure::NotALog(_) => ParseError { why: NO_LOG.to_string() },
        SarifFailure::Unplaced(error) => error,
    })?;
    Ok(hits.into_iter().map(|hit| Hit { what: WHAT.to_string(), ..hit }).collect())
}

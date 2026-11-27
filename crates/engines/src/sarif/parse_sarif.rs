//! `parse_sarif`: a SARIF 2.1 log, read into the hits of one rule.

use super::read_sarif::read_sarif;
use super::sarif_failure::SarifFailure;
use crate::types::{Hit, ParseError};
use procio::Output;
use std::path::Path;

/// The hits of rule `check` in `out`, as `read_sarif` reads them; when it fails, either failure's `ParseError` (stdout
/// that is not a SARIF log, or a checked result outside `folder`). An entry whose engine prints SARIF sets its reader
/// to `parse_sarif`, or to a reader built on `read_sarif` (`parse_gitleaks` is).
pub fn parse_sarif(out: &Output, check: &str, cwd: &Path, folder: &Path) -> Result<Vec<Hit>, ParseError> {
    read_sarif(out, check, cwd, folder).map_err(|failure| match failure {
        SarifFailure::NotALog(error) | SarifFailure::Unplaced(error) => error,
    })
}

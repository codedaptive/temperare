//! `parse_hadolint`: hadolint's SARIF log, read into the hits of one rule, or why it is no answer.

use crate::sarif::parse_sarif;
use crate::types::{Hit, ParseError};
use procio::Output;
use std::path::Path;

/// What hadolint 2.15.1 prints to stderr when it cannot read its config (measured). It then runs on its defaults and
/// still prints a SARIF log, so that log answers a config tm did not pin.
const CONFIG_ERROR: &str = "Error parsing your config file";

/// The code hadolint gives a Dockerfile it cannot parse. That file's other findings are then missing (measured), so a
/// rule would read clean on it.
const PARSE_FAILURE: &str = "DL1000";

/// The hits of `check` in `out`, read by `parse_sarif` with `cwd` as hadolint's working directory and `folder` as the
/// rule's. Any exit code with a SARIF log on stdout is an answer, except that the run is a `ParseError`:
/// - when stderr holds hadolint's config error: hadolint could not read its config and ran on its defaults
///   (`parse_failed` then shows hadolint's own stderr);
/// - when the log holds a `DL1000` result under `folder`: hadolint could not parse that Dockerfile, so its other
///   findings are missing; the why names the first one's path and line and quotes hadolint's text;
/// - when a result of `check`, or a `DL1000` result, names a file not under `folder`: `parse_sarif`'s `ParseError`,
///   naming its URI as printed.
///
/// stdout that is not one SARIF log is `parse_sarif`'s `ParseError`; that covers a config `format:` key, which makes
/// hadolint print a second format glued before the log.
pub fn parse_hadolint(out: &Output, check: &str, cwd: &Path, folder: &Path) -> Result<Vec<Hit>, ParseError> {
    if String::from_utf8_lossy(&out.stderr).contains(CONFIG_ERROR) {
        return Err(ParseError { why: "hadolint could not read its config and would run on its defaults".to_string() });
    }
    if let Some(failure) = parse_sarif(out, PARSE_FAILURE, cwd, folder)?.first() {
        let line = failure.line.map(|line| format!(":{line}")).unwrap_or_default();
        return Err(ParseError {
            why: format!(
                "hadolint could not parse {}{line}, so its other findings there are missing: {}",
                failure.path.as_str(),
                failure.what
            ),
        });
    }
    parse_sarif(out, check, cwd, folder)
}

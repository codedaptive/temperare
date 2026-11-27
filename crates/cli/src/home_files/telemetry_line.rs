//! `telemetry_line`: one check's line in the telemetry file.

use serde_json::Value;

use super::{TelemetryRun, utc_rfc3339};

/// One JSON object, without a newline, keys in this order: `format` (1), `time` (UTC RFC 3339 to the second, `Z`),
/// `version`, `folder` (lossy UTF-8), `head`, `base` (null without one), `kind` (`quality`, `security` or null),
/// `paths` (`[]` for the whole tree), `exit`, `engines` (the report's engines) and `rules` (the report's record).
/// Strings are JSON-escaped. A value that cannot be serialized is written as null (`kind`) or `[]` (`paths`,
/// `engines`, `rules`), so the line is still one JSON object.
pub(crate) fn telemetry_line(
    run: &TelemetryRun,
    unix_secs: u64,
    version: &str,
    head: &str,
    report: &report::Report,
) -> String {
    // Built by hand so the keys keep this order; serde_json's map would sort them.
    let folder = Value::String(run.folder.to_string_lossy().into_owned());
    let base = run.base.map_or(Value::Null, |base| Value::String(base.to_owned()));
    let kind = serde_json::to_string(&run.kind).unwrap_or_else(|_| "null".to_owned());
    let paths = serde_json::to_string(run.paths).unwrap_or_else(|_| "[]".to_owned());
    let engines = serde_json::to_string(&report.engines).unwrap_or_else(|_| "[]".to_owned());
    let rules = serde_json::to_string(&report.record).unwrap_or_else(|_| "[]".to_owned());
    format!(
        "{{\"format\":1,\"time\":\"{}\",\"version\":{},\"folder\":{folder},\"head\":{},\"base\":{base},\
         \"kind\":{kind},\"paths\":{paths},\"exit\":{},\"engines\":{engines},\"rules\":{rules}}}",
        utc_rfc3339(unix_secs),
        Value::String(version.to_owned()),
        Value::String(head.to_owned()),
        run.exit,
    )
}

//! `serde_name`: the serde name of a unit enum value, from its JSON string.

/// The serde name of an Engine, Kind or Severity, read from its JSON string (empty when the value is not a string or
/// is an error); cli names no serde trait (S-16).
pub(crate) fn serde_name(value: serde_json::Result<serde_json::Value>) -> String {
    value.ok().and_then(|v| v.as_str().map(str::to_string)).unwrap_or_default()
}

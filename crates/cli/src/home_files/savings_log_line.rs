//! `savings_log_line`: one check's line in the savings log.

use std::path::Path;

use super::utc_rfc3339;

/// One JSON line, without a newline, keys in this order: `time` (UTC RFC 3339 to the second, `Z`), `folder` (lossy
/// UTF-8, JSON-escaped), `verb` (`check`), `output_tokens` and `baseline_tokens`.
pub fn savings_log_line(unix_secs: u64, folder: &Path, output_tokens: u64, baseline_tokens: u64) -> String {
    // Built by hand so the keys keep this order; serde_json's map would sort them.
    let folder = serde_json::Value::String(folder.to_string_lossy().into_owned());
    format!(
        "{{\"time\":\"{}\",\"folder\":{folder},\
         \"verb\":\"check\",\"output_tokens\":{output_tokens},\"baseline_tokens\":{baseline_tokens}}}",
        utc_rfc3339(unix_secs)
    )
}

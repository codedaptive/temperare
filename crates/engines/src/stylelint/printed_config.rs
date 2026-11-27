//! `printed_config`: what `stylelint --print-config` printed for one file, read for the check.

use crate::printed_start::printed_start;
use crate::types::Stop;
use procio::Output;
use serde_json::Value;

/// Whether the config stylelint printed for `file` (relative to the rule's folder) turns on `check`, or why the rule
/// stops there:
/// - a probe that did not exit 0 is `Failed` with its exit code and stylelint's stderr (a config that does not load
///   says why there);
/// - stdout that is not a JSON object is `Failed` quoting what stylelint printed;
/// - a truthy `fix` is `Failed`: stylelint would rewrite the checked files, and no option turns a config's `fix` off;
/// - a truthy `quiet` is `Failed`: it drops warning-severity findings, a hidden exception;
/// - else `true` when `rules.<check>` is present and not null (stylelint prints each rule's settings as an array and a
///   rule the config turns off as null; an unknown name prints as an array too, and the check run then says
///   `Unknown rule`), `false` when it is absent or null.
pub(crate) fn printed_config(out: &Output, check: &str, file: &str) -> Result<bool, Stop> {
    if out.code != Some(0) {
        return Err(Stop::Failed { code: out.code, stderr: String::from_utf8_lossy(&out.stderr).into_owned() });
    }
    let Ok(Value::Object(config)) = serde_json::from_slice(&out.stdout) else {
        let stderr = format!("stylelint --print-config printed no config for {file}: {}", printed_start(out));
        return Err(Stop::Failed { code: out.code, stderr });
    };
    if truthy(config.get("fix")) {
        return Err(Stop::Failed {
            code: None,
            stderr: format!(
                "stylelint's config for {file} turns on fix, which rewrites the checked files; cctao never edits the code it checks"
            ),
        });
    }
    if truthy(config.get("quiet")) {
        return Err(Stop::Failed {
            code: None,
            stderr: format!(
                "stylelint's config for {file} turns on quiet, which drops warning-severity findings; exceptions go in the rule file"
            ),
        });
    }
    Ok(config.get("rules").and_then(|rules| rules.get(check)).is_some_and(|rule| !rule.is_null()))
}

/// JavaScript's truthiness, as stylelint tests its options: absent, null, false, 0 and "" are off; anything else, a
/// non-empty string included, is on.
fn truthy(value: Option<&Value>) -> bool {
    match value {
        None | Some(Value::Null) => false,
        Some(Value::Bool(on)) => *on,
        Some(Value::Number(n)) => n.as_f64() != Some(0.0),
        Some(Value::String(s)) => !s.is_empty(),
        Some(_) => true,
    }
}

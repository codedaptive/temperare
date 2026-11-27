//! `config_data`: the YAML config cctao hands yamllint with `-d` for one rule.

use std::path::Path;

/// The `-d` text `{extends: <base>, ignore: [], rules: {"<check>": {ignore-from-file: []}}}`, its parts:
/// - `extends: <base>` loads the repository's config (`yamllint_config`) as the base, so its rules and options hold;
///   `<base>` is the path as a YAML double-quoted scalar, or the plain word `default` (yamllint's stock config) when
///   `base` is None.
/// - `ignore: []` replaces the base's top-level `ignore` or `ignore-from-file`, one inherited through an `extends`
///   chain included: yamllint applies the `-d` text's own `ignore` after `extends` has copied the base's, so no file cctao
///   passes is dropped (S-67 (1); this form is the one measured, fixtures/fixtures.txt).
/// - naming the check under `rules` enables it even where the base disables it: a base `false` or `disable` is
///   replaced, so the check runs with its own option defaults at level error; a base table keeps its options and its
///   level (both measured, fixtures/fixtures.txt).
/// - `ignore-from-file: []` clears the check's own `ignore-from-file` (S-67 (1)), so a path the repository's config
///   hides from that rule is still checked (tests/yamllint.rs, "check-ignore-from-file").
///
/// The check key is double-quoted too, so PyYAML, which reads YAML 1.1, takes `yes` or `on` as the string, not a
/// boolean. The check is a rule id already (`yamllint_refusal`), so it needs no escaping. Err(why) when `base` is not
/// valid UTF-8, since the text handed yamllint is UTF-8.
pub(crate) fn config_data(base: Option<&Path>, check: &str) -> Result<String, String> {
    let base = match base {
        None => "default".to_string(),
        Some(path) => {
            let text = path.to_str().ok_or_else(|| format!("yamllint config path {} is not valid UTF-8", path.display()))?;
            quoted(text)
        }
    };
    Ok(format!("{{extends: {base}, ignore: [], rules: {{\"{check}\": {{ignore-from-file: []}}}}}}"))
}

/// `text` as a YAML double-quoted scalar: `\` as `\\`, `"` as `\"`, and every char below U+0020 or from U+007F to
/// U+009F as `\xHH`. PyYAML refuses most of those raw ("unacceptable character", measured on yamllint 1.38.0 for
/// U+007F and U+0080) and would fold a raw line break; escaped, they name the path as it is.
fn quoted(text: &str) -> String {
    let mut out = String::from("\"");
    for ch in text.chars() {
        match ch {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\u{0}'..='\u{1f}' | '\u{7f}'..='\u{9f}' => out.push_str(&format!("\\x{:02X}", u32::from(ch))),
            _ => out.push(ch),
        }
    }
    out.push('"');
    out
}

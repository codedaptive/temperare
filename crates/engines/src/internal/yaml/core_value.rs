//! `core_value`: a scalar as YAML 1.2's core schema reads it, for yaml_entries (a repeated key, a tag its text does not
//! fit or cctao does not read) and markdownlint_cli2's yaml_settings (a plain scalar's truthiness). Tested through
//! yaml_entries_tests.rs and markdownlint_cli2/yaml_settings_tests.rs, so no test file of its own.

use saphyr_parser::{ScalarStyle, Tag};

/// The value the core schema reads for the scalar with `text`, in `style`, under `tag` if any, written so that two
/// scalars with one value give one string: `null`, `bool true`, `number <value>` or `string <decoded text>`.
/// - A core tag decides when present: `!!null`, `!!bool`, `!!int` and `!!float` read the text as that type, and text
///   that is not one is an `Err` naming the tag; `!!str` reads it as a string; any other `!!` tag (`!!seq`,
///   `!!binary`, `!!timestamp`, ...) is an `Err` naming it, since cctao does not read it.
/// - Any other tag (`!foo`, the non-specific `!`) reads the decoded text as a string, as dart does: `!foo a` is `a`,
///   `! 1` is `"1"`, and `!foo 1` is not `1`.
/// - An untagged plain scalar resolves as the core schema does: null forms, booleans, ints in decimal, `0o` octal or
///   `0x` hex, floats with `.inf` and `.nan`; anything else is a string. Any other untagged scalar is a string.
///
/// So `1`, `0x1` and `1.0` are one value, `"a"` and `a` are one, `"1"` and `1` are two, and two `.nan` are one (all
/// as dart 3.14.1 reads keys, dart/fixtures/commands.txt, "YAML forms" 10 and 11). Numbers compare as `f64`: two huge
/// distinct ints that read as one value make a refusal, never a miss.
pub(crate) fn core_value(text: &str, style: ScalarStyle, tag: Option<&Tag>) -> Result<String, String> {
    let string = || format!("string {text}");
    let Some(suffix) = tag.filter(|tag| tag.is_yaml_core_schema()).map(|tag| tag.suffix.as_str()) else {
        let resolves = tag.is_none() && style == ScalarStyle::Plain;
        return Ok(resolves.then(|| resolved(text)).flatten().unwrap_or_else(string));
    };
    let read = match suffix {
        "null" => null(text),
        "bool" => boolean(text),
        "int" => int(text),
        "float" => float(text),
        "str" => return Ok(string()),
        _ => return Err(format!("a `!!{suffix}` tag on a scalar, which cctao does not read")),
    };
    read.ok_or_else(|| format!("a `!!{suffix}` tag on `{text}`, which does not read as one"))
}

/// An untagged plain scalar's core-schema value, None when it reads as a string.
fn resolved(text: &str) -> Option<String> {
    null(text).or_else(|| boolean(text)).or_else(|| int(text)).or_else(|| float(text))
}

/// `text` as the core schema's null.
fn null(text: &str) -> Option<String> {
    matches!(text, "" | "~" | "null" | "Null" | "NULL").then(|| "null".to_string())
}

/// `text` as a core-schema boolean.
fn boolean(text: &str) -> Option<String> {
    match text {
        "true" | "True" | "TRUE" => Some("bool true".to_string()),
        "false" | "False" | "FALSE" => Some("bool false".to_string()),
        _ => None,
    }
}

/// `text` as a core-schema int: `[-+]?[0-9]+`, `0o[0-7]+` or `0x[0-9a-fA-F]+`.
fn int(text: &str) -> Option<String> {
    let value = match (text.strip_prefix("0o"), text.strip_prefix("0x")) {
        (Some(octal), _) => radix(octal, 8),
        (_, Some(hex)) => radix(hex, 16),
        _ => digits(text.strip_prefix(['-', '+']).unwrap_or(text)).then(|| text.parse().ok()).flatten(),
    };
    value.map(number)
}

/// `text` as a core-schema float: `[-+]?(\.[0-9]+|[0-9]+(\.[0-9]*)?)([eE][-+]?[0-9]+)?`, `[-+]?.inf` in its three
/// cases, or `.nan` in its three. Rust's own parse reads more (`inf`, `nan`), so the shape is checked first.
fn float(text: &str) -> Option<String> {
    let body = text.strip_prefix(['-', '+']).unwrap_or(text);
    let value = match text {
        ".nan" | ".NaN" | ".NAN" => Some(f64::NAN),
        _ if matches!(body, ".inf" | ".Inf" | ".INF") => {
            Some(if text.starts_with('-') { f64::NEG_INFINITY } else { f64::INFINITY })
        }
        _ => {
            let (mantissa, exponent) = body.split_once(['e', 'E']).unwrap_or((body, "0"));
            let mantissa_fits = match mantissa.split_once('.') {
                Some((whole, fraction)) => {
                    let after = fraction.is_empty() || digits(fraction);
                    (digits(whole) && after) || (whole.is_empty() && digits(fraction))
                }
                None => digits(mantissa),
            };
            let fits = mantissa_fits && digits(exponent.strip_prefix(['-', '+']).unwrap_or(exponent));
            fits.then(|| text.parse().ok()).flatten()
        }
    };
    value.map(number)
}

/// Whether `part` is one or more ASCII digits and nothing else.
fn digits(part: &str) -> bool {
    !part.is_empty() && part.bytes().all(|b| b.is_ascii_digit())
}

/// `digits` in base `radix`, at least one digit and nothing else.
fn radix(digits: &str, radix: u32) -> Option<f64> {
    if digits.is_empty() {
        return None;
    }
    digits.chars().try_fold(0.0, |value, c| c.to_digit(radix).map(|d| value * f64::from(radix) + f64::from(d)))
}

/// A number as a value; `+ 0.0` makes `-0` and `0` one value.
fn number(value: f64) -> String {
    format!("number {}", value + 0.0)
}

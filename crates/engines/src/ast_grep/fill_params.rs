//! `fill_params`: a rule YAML template with each `{{name}}` replaced by the rule's param of that name.

use serde::Serialize;
use serde_json::Value;

use crate::ParamError;

/// `yaml` with each `{{name}}` (the name trimmed) replaced from `params`, which must serialize to an object: a string
/// param by itself as written, a list of strings by its items regex-escaped and joined by `|`, so the list drops into
/// a `regex:` alternation and matches each item literally. A `{{` with no closing `}}` stays as written. A `{{name}}`
/// the params do not give is `Unknown`, checked first; then a param no `{{name}}` names is `Bad`, so a misspelt name
/// is never silently dropped (`unused_why`).
pub fn fill_params<P: Serialize>(yaml: &str, params: &P) -> Result<String, ParamError> {
    let Ok(Value::Object(params)) = serde_json::to_value(params) else {
        return Err(bad("params", "not a table of named params"));
    };
    let mut filled = String::with_capacity(yaml.len());
    let mut names: Vec<&str> = Vec::new();
    let mut rest = yaml;
    while let Some(open) = rest.find("{{") {
        let after = &rest[open + 2..];
        let Some(close) = after.find("}}") else { break };
        let name = after[..close].trim();
        filled.push_str(&rest[..open]);
        filled.push_str(&param_text(name, params.get(name))?);
        if !names.contains(&name) {
            names.push(name);
        }
        rest = &after[close + 2..];
    }
    filled.push_str(rest);
    match params.keys().find(|key| !names.contains(&key.as_str())) {
        Some(key) => Err(bad(key, &unused_why(key, &names))),
        None => Ok(filled),
    }
}

/// Why the param `key` is refused: the YAML has no `{{key}}`, and its placeholders, in order of first appearance, are
/// listed (or it has none).
fn unused_why(key: &str, names: &[&str]) -> String {
    if names.is_empty() {
        return format!("the rule YAML has no {{{{{key}}}}} and no placeholders");
    }
    let listed = names.iter().map(|name| format!("{{{{{name}}}}}")).collect::<Vec<_>>().join(", ");
    format!("the rule YAML has no {{{{{key}}}}}; its placeholders are {listed}")
}

/// The text a param fills its `{{name}}` with.
fn param_text(name: &str, value: Option<&Value>) -> Result<String, ParamError> {
    match value {
        None => Err(ParamError::Unknown { name: name.to_string() }),
        Some(Value::String(text)) => Ok(text.clone()),
        Some(Value::Array(items)) => items
            .iter()
            .map(|item| item.as_str().map(regex_syntax::escape).ok_or_else(|| bad(name, "a list item is not a string")))
            .collect::<Result<Vec<_>, _>>()
            .map(|escaped| escaped.join("|")),
        Some(_) => Err(bad(name, "not a string or a list of strings")),
    }
}

fn bad(name: &str, why: &str) -> ParamError {
    ParamError::Bad { name: name.to_string(), why: why.to_string() }
}

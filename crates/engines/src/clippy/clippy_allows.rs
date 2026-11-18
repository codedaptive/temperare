//! `clippy_allows`: the lints a clippy rule's params name under `allow`.

use serde::Serialize;
use serde_json::Value;

use crate::ParamError;

/// The `allow` list of `params`, in order; no params or no `allow` is an empty list. `allow` is the only param a
/// clippy rule reads, so any other key is `Bad` rather than ignored: a misspelt `allow` would otherwise report the
/// lints it was meant to allow. `params` must serialize to an object and `allow` to a list of strings.
pub(crate) fn clippy_allows<P: Serialize>(params: &P) -> Result<Vec<String>, ParamError> {
    let Ok(Value::Object(params)) = serde_json::to_value(params) else {
        return Err(bad("params", "not a table of named params"));
    };
    if let Some(key) = params.keys().find(|key| *key != "allow") {
        return Err(bad(key, "a clippy rule reads only the `allow` param"));
    }
    let Some(allow) = params.get("allow") else { return Ok(Vec::new()) };
    let not_strings = || bad("allow", "must be a list of strings");
    let Value::Array(items) = allow else { return Err(not_strings()) };
    items.iter().map(|item| item.as_str().map(str::to_string).ok_or_else(not_strings)).collect()
}

fn bad(name: &str, why: &str) -> ParamError {
    ParamError::Bad { name: name.to_string(), why: why.to_string() }
}

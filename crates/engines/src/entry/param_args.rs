//! `param_args`: a rule's params as the arguments its engine takes them as.

use super::{Entry, ParamArg, Params, Threshold};
use crate::ParamError;
use serde::Serialize;
use serde_json::{Map, Value};
use std::ffi::OsString;

/// For `Params::Args(list)` and the `args` list of `Params::Thresholds`: each listed param's items, in the list's
/// order, each one argument `{flag}{item}`; a listed param the rule does not give adds nothing. A key naming one of
/// a `Thresholds` entry's thresholds gives no argument and is not refused here (`threshold_of` reads it). `params`
/// must serialize to an object (else `Bad` named `params`). A key neither list names is `Bad` rather than ignored,
/// since a misspelt name would otherwise drop what the rule asked for: its why is `a`/`an <name> rule reads only the
/// <names> param` (every arg name, then every threshold name), or `a`/`an <name> rule takes no params` when both lists
/// are empty. A listed param that is not a list of strings is `Bad`. For `Params::Flow`, the params must be an
/// object whose keys are all in its `names` (else `Bad` with the same why, over `names`), then its `check` decides,
/// its error returned as it is; it gives no arguments. `Params::Template` gives no arguments whatever the params.
pub(crate) fn param_args<P: Serialize>(entry: &Entry, params: &P) -> Result<Vec<OsString>, ParamError> {
    let (list, names, check) = match entry.params {
        Params::Args(list) => (list, names_of(list, &[]), None),
        Params::Thresholds { args, thresholds, .. } => (args, names_of(args, thresholds), None),
        Params::Flow { names, check } => (&[][..], names.to_vec(), Some(check)),
        Params::Template => return Ok(Vec::new()),
    };
    let Ok(Value::Object(params)) = serde_json::to_value(params) else {
        return Err(bad("params", "not a table of named params"));
    };
    if let Some(key) = params.keys().find(|key| !names.contains(&key.as_str())) {
        return Err(bad(key, &unlisted_why(entry.name, &names)));
    }
    if let Some(check) = check {
        check(&params)?;
    }
    let mut args = Vec::new();
    for arg in list {
        args.extend(items(&params, arg)?);
    }
    Ok(args)
}

/// Every arg name, then every threshold name.
fn names_of(args: &[ParamArg], thresholds: &[Threshold]) -> Vec<&'static str> {
    args.iter().map(|arg| arg.name).chain(thresholds.iter().map(|threshold| threshold.name)).collect()
}

/// The arguments of one listed param.
fn items(params: &Map<String, Value>, arg: &ParamArg) -> Result<Vec<OsString>, ParamError> {
    let Some(value) = params.get(arg.name) else { return Ok(Vec::new()) };
    let not_strings = || bad(arg.name, "must be a list of strings");
    let Value::Array(items) = value else { return Err(not_strings()) };
    items
        .iter()
        .map(|item| item.as_str().map(|item| format!("{}{item}", arg.flag).into()).ok_or_else(not_strings))
        .collect()
}

/// Why a key no list names is refused: the names, each in backticks, joined by ", "; or that the engine takes none.
/// The article is `an` before a name starting with an ASCII vowel letter: every such engine name (ast-grep, eslint,
/// actionlint) is said with a vowel sound.
fn unlisted_why(name: &str, names: &[&str]) -> String {
    let article = if name.starts_with(['a', 'e', 'i', 'o', 'u']) { "an" } else { "a" };
    if names.is_empty() {
        return format!("{article} {name} rule takes no params");
    }
    let names = names.iter().map(|param| format!("`{param}`")).collect::<Vec<_>>().join(", ");
    format!("{article} {name} rule reads only the {names} param")
}

fn bad(name: &str, why: &str) -> ParamError {
    ParamError::Bad { name: name.to_string(), why: why.to_string() }
}

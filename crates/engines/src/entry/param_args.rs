//! `param_args`: a rule's params as the arguments its engine takes them as.

use super::{Entry, ParamArg, Params};
use crate::ParamError;
use serde::Serialize;
use serde_json::{Map, Value};
use std::ffi::OsString;

/// For `Params::Args(list)` and the `args` list of `Params::Thresholds`: each listed param's items, in the list's
/// order, each one argument `{flag}{item}`; a listed param the rule does not give adds nothing. A key naming one of
/// a `Thresholds` entry's thresholds gives no argument and is not refused here (`threshold_of` reads it). `params`
/// must serialize to an object (else `Bad` named `params`). A key neither list names is `Bad` rather than ignored,
/// since a misspelt name would otherwise drop what the rule asked for: its why is `a <name> rule reads only the
/// <names> param` (every arg name, then every threshold name), or `a <name> rule takes no params` when both lists
/// are empty. A listed param that is not a list of strings is `Bad`. Every other `Params` kind gives no arguments.
pub(crate) fn param_args<P: Serialize>(entry: &Entry, params: &P) -> Result<Vec<OsString>, ParamError> {
    let (list, thresholds) = match entry.params {
        Params::Args(list) => (list, &[][..]),
        Params::Thresholds { args, thresholds, .. } => (args, thresholds),
        Params::Config(_) | Params::Template => return Ok(Vec::new()),
    };
    let names: Vec<&str> = list.iter().map(|arg| arg.name).chain(thresholds.iter().map(|t| t.name)).collect();
    let Ok(Value::Object(params)) = serde_json::to_value(params) else {
        return Err(bad("params", "not a table of named params"));
    };
    if let Some(key) = params.keys().find(|key| !names.contains(&key.as_str())) {
        return Err(bad(key, &unlisted_why(entry.name, &names)));
    }
    let mut args = Vec::new();
    for arg in list {
        args.extend(items(&params, arg)?);
    }
    Ok(args)
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
fn unlisted_why(name: &str, names: &[&str]) -> String {
    if names.is_empty() {
        return format!("a {name} rule takes no params");
    }
    let names = names.iter().map(|param| format!("`{param}`")).collect::<Vec<_>>().join(", ");
    format!("a {name} rule reads only the {names} param")
}

fn bad(name: &str, why: &str) -> ParamError {
    ParamError::Bad { name: name.to_string(), why: why.to_string() }
}

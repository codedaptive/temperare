//! `threshold_of`: the threshold a rule sets, and its number.

use super::{Entry, Params, Threshold};
use crate::ParamError;
use serde::Serialize;
use serde_json::{Map, Value};

/// For `Params::Thresholds`: the one threshold the rule's `params` give, with its number; `None` when they give
/// none, and for every other `Params` kind. `params` must serialize to an object (else `Bad` named `params`). Each
/// threshold given, in the entry's list order, is `Bad` when an earlier one was given (one threshold per rule), when
/// its `check` is not the rule's `check` (read with every `-` as `_`), or when its value is not a whole number from
/// 1 to its `max`. Keys that name no threshold are left to `param_args`.
pub(crate) fn threshold_of<P: Serialize>(
    entry: &Entry,
    check: &str,
    params: &P,
) -> Result<Option<(Threshold, u32)>, ParamError> {
    let Params::Thresholds { thresholds, .. } = entry.params else { return Ok(None) };
    let Ok(Value::Object(params)) = serde_json::to_value(params) else {
        return Err(bad("params", "not a table of named params"));
    };
    let check = check.replace('-', "_");
    let mut found = None;
    for threshold in thresholds.iter().filter(|threshold| params.contains_key(threshold.name)) {
        if found.is_some() {
            return Err(bad(threshold.name, "only one threshold per rule"));
        }
        found = Some((*threshold, number(&params, threshold, &check)?));
    }
    Ok(found)
}

/// The number one given threshold holds, when it is for `check` and in range.
fn number(params: &Map<String, Value>, threshold: &Threshold, check: &str) -> Result<u32, ParamError> {
    if threshold.check != check {
        return Err(bad(threshold.name, &format!("read only with check `{}`", threshold.check)));
    }
    params
        .get(threshold.name)
        .and_then(Value::as_u64)
        .and_then(|n| u32::try_from(n).ok())
        .filter(|n| (1..=threshold.max).contains(n))
        .ok_or_else(|| bad(threshold.name, &format!("must be a whole number from 1 to {}", threshold.max)))
}

fn bad(name: &str, why: &str) -> ParamError {
    ParamError::Bad { name: name.to_string(), why: why.to_string() }
}

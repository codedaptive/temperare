//! `split_count_params`: the rule's count-mode params taken out of the params that fill its YAML.

use serde::Serialize;
use serde_json::{Map, Value};

use super::CountMode;

/// The one value `count` takes.
const SETUP_VS_TESTS: &str = "setup-vs-tests";

/// `params` serialized to an object, as `fill_params` reads them, split in two: the count mode (Some when `count` is
/// given) and every other param, for the fill. The checks run in this order, and Err is the first refusal's stderr
/// text: params that are not an object; `count` other than "setup-vs-tests"; `shared_setup` without `count`;
/// `shared_setup` not a list of strings; a bad glob in it (the text of `Scope::build`'s Err). `shared_setup`'s globs
/// are relative to the rule file's folder; absent or empty, nothing is exempt.
pub(crate) fn split_count_params<P: Serialize>(
    params: &P,
    check: &str,
) -> Result<(Option<CountMode>, Map<String, Value>), String> {
    let Ok(Value::Object(mut rest)) = serde_json::to_value(params) else {
        return Err(format!("param params in {check}: not a table of named params"));
    };
    let count = rest.remove("count");
    let shared_setup = rest.remove("shared_setup");
    if count.as_ref().is_some_and(|value| value.as_str() != Some(SETUP_VS_TESTS)) {
        return Err(format!("param count in {check}: takes only \"{SETUP_VS_TESTS}\""));
    }
    let mode = match (count, shared_setup) {
        (None, None) => None,
        (None, Some(_)) => {
            return Err(format!("param shared_setup in {check}: needs count = \"{SETUP_VS_TESTS}\""));
        }
        (Some(_), None) => Some(CountMode { shared_setup: None }),
        (Some(_), Some(value)) => Some(CountMode { shared_setup: shared_scope(&value, check)? }),
    };
    Ok((mode, rest))
}

/// The exempt files' scope from `shared_setup`'s value; None for an empty list.
fn shared_scope(value: &Value, check: &str) -> Result<Option<rules::Scope>, String> {
    let globs = value
        .as_array()
        .and_then(|items| items.iter().map(|item| item.as_str().map(str::to_string)).collect::<Option<Vec<_>>>())
        .ok_or_else(|| format!("param shared_setup in {check}: not a list of globs"))?;
    if globs.is_empty() {
        return Ok(None);
    }
    rules::Scope::build(&globs, &[], None)
        .map(Some)
        .map_err(|err| format!("param shared_setup in {check}: {err}"))
}

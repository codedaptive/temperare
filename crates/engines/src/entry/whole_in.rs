//! `whole_in`: one param read as a whole number in a range.

use crate::ParamError;
use serde_json::{Map, Value};

/// The value at `name` as a whole number from `min` to `max`. Anything else, a missing key included, is `Bad` named
/// `name` with why `must be a whole number from <min> to <max>`; a caller that wants another why for a missing key
/// checks for it first.
pub(crate) fn whole_in(params: &Map<String, Value>, name: &str, min: u32, max: u32) -> Result<u32, ParamError> {
    params
        .get(name)
        .and_then(Value::as_u64)
        .and_then(|n| u32::try_from(n).ok())
        .filter(|n| (min..=max).contains(n))
        .ok_or_else(|| ParamError::Bad {
            name: name.to_string(),
            why: format!("must be a whole number from {min} to {max}"),
        })
}

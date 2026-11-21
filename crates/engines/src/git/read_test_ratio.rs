//! `read_test_ratio`: a git rule's params, checked and read.

use super::TestRatio;
use crate::entry::whole_in;
use crate::ParamError;
use rules::Scope;
use serde_json::{Map, Value};

/// Reads `max_ratio` (a number from 0.1 to 100), `floor` (a whole number from 0 to 100000) and `test_paths` (a
/// non-empty list of globs), in that order; the first one missing or wrong is `Bad`. Other keys are left to the
/// caller.
pub(crate) fn read_test_ratio(params: &Map<String, Value>) -> Result<TestRatio, ParamError> {
    let max_ratio = given(params, "max_ratio")?
        .as_f64()
        .filter(|x| x.is_finite() && (0.1..=100.0).contains(x))
        .ok_or_else(|| bad("max_ratio", "must be a number from 0.1 to 100"))?;
    given(params, "floor")?;
    let floor = whole_in(params, "floor", 0, 100_000)?;
    let globs = given(params, "test_paths")?
        .as_array()
        .filter(|list| !list.is_empty())
        .and_then(|list| list.iter().map(|g| g.as_str().map(str::to_string)).collect::<Option<Vec<_>>>())
        .ok_or_else(|| bad("test_paths", "must be a list of glob strings"))?;
    let test_paths = Scope::build(&globs, &[], None).map_err(|text| bad("test_paths", &format!("bad glob: {text}")))?;
    Ok(TestRatio { max_ratio, floor, test_paths })
}

/// The param `name`, or `Bad` because a git rule needs every one of its params.
fn given<'a>(params: &'a Map<String, Value>, name: &str) -> Result<&'a Value, ParamError> {
    params.get(name).ok_or_else(|| bad(name, "a git rule needs it"))
}

fn bad(name: &str, why: &str) -> ParamError {
    ParamError::Bad { name: name.to_string(), why: why.to_string() }
}

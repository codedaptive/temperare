//! `Params`: which of a rule's `params` an engine accepts, and how each reaches it.

use super::{ParamArg, Reach, Threshold};
use crate::ParamError;
use serde_json::{Map, Value};

/// How a rule's `params` table reaches the engine. A param is never passed through as an argument the rule file
/// wrote, and never dropped: each one an engine accepts is named here.
#[derive(Debug, Clone, Copy)]
pub enum Params {
    /// Each listed param is a list of strings, each item one argument `{flag}{item}` (`param_args`); a param not
    /// listed is an error, so a misspelt name is never silently dropped. An engine that takes no params gives the
    /// empty list, so any param makes its rule fail.
    Args(&'static [ParamArg]),
    /// `args` as `Args`; each threshold is a param holding one whole number for one check (`threshold_of`), which
    /// reaches the engine by `reach` (`threshold_run`). A param in neither list is an error.
    Thresholds { args: &'static [ParamArg], thresholds: &'static [Threshold], reach: Reach },
    /// The params fill the `{{name}}`s of the engine's check file (ast-grep); the entry's own flow does it.
    Template,
    /// The params the entry's own flow reads. A key not in `names` is refused, and `check` refuses a value the flow
    /// could not use, both before anything runs (`param_args`); they reach the engine only through the flow.
    Flow { names: &'static [&'static str], check: fn(&Map<String, Value>) -> Result<(), ParamError> },
}

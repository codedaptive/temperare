//! `param_why`: the stderr text of a rule that stopped on its params.

use crate::ParamError;

/// `<name> param <p>: <why>` for a `Bad` param, `<name> param <p>: unknown` for an `Unknown` one; `name` is the
/// entry's name.
pub(crate) fn param_why(name: &str, err: ParamError) -> String {
    match err {
        ParamError::Unknown { name: param } => format!("{name} param {param}: unknown"),
        ParamError::Bad { name: param, why } => format!("{name} param {param}: {why}"),
    }
}

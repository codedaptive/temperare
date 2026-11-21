//! `read_case_params`: the `params` a baseline set rule's failing or passing case runs with.

use super::check_not_option::check_not_option;
use super::check_param_types::check_param_types;
use super::param_type::ParamType;
use super::wrong_type::wrong_type;
use crate::Problem;
use toml::de::DeTable;

/// A Problem in `out` for each fault of the `params` in the raw `case` table of the set rule `id`: a `params` that is
/// not a table is the wrong type; in a table, each `allow` item that starts with '-' (as in a rule's own params), and
/// each typed required param of `types` (as read_required_params gives them) given with the wrong type. Absent
/// `params` is no Problem.
pub(crate) fn read_case_params(
    text: &str,
    case: &DeTable,
    id: &str,
    types: &[(String, Option<ParamType>)],
    out: &mut Vec<Problem>,
) {
    let Some(value) = case.get("params") else {
        return;
    };
    let Some(params) = value.get_ref().as_table() else {
        out.push(wrong_type(text, "params", value, "a table"));
        return;
    };
    check_not_option(text, case, out);
    check_param_types(text, id, types, params, out);
}

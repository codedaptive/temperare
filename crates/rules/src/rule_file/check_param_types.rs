//! `check_param_types`: the params an `[[override]]` gives a baseline rule, against the types the set declares.

use super::param_type::ParamType;
use super::problem_at::problem_at;
use crate::Problem;
use toml::de::DeTable;

/// A Problem for each typed required param of the baseline rule `id` (`types`, as read_required_params gives them)
/// that `params` gives with the wrong type: a `String` param must be a string, a `Strings` param an array whose every
/// item is a string (a Problem at each item that is not). Untyped params, and params not given, are not checked.
pub(crate) fn check_param_types(
    text: &str,
    id: &str,
    types: &[(String, Option<ParamType>)],
    params: &DeTable,
    out: &mut Vec<Problem>,
) {
    for (name, ty) in types {
        let (Some(ty), Some(value)) = (ty, params.get(name.as_str())) else {
            continue;
        };
        let start = format!("param `{name}` of baseline rule \"{id}\" must be");
        match (ty, value.get_ref().as_array()) {
            (ParamType::String, _) if value.get_ref().as_str().is_none() => out.push(problem_at(
                text,
                value.span().start,
                format!("{start} a string, not {}", value.get_ref().type_str()),
                &format!("Write `{name}` as a string, e.g. `{name} = \"...\"`."),
            )),
            (ParamType::Strings, None) => out.push(problem_at(
                text,
                value.span().start,
                format!("{start} a list of strings, not {}", value.get_ref().type_str()),
                &list_fix(name),
            )),
            (ParamType::Strings, Some(items)) => {
                out.extend(items.iter().enumerate().filter(|(_, item)| item.get_ref().as_str().is_none()).map(
                    |(i, item)| {
                        problem_at(
                            text,
                            item.span().start,
                            format!("{start} a list of strings; item {} is {}", i + 1, item.get_ref().type_str()),
                            &list_fix(name),
                        )
                    },
                ));
            }
            _ => {}
        }
    }
}

/// The fix for a `Strings` param given with the wrong type.
fn list_fix(name: &str) -> String {
    format!("Write `{name}` as a list of strings, e.g. `{name} = [\"...\"]`.")
}

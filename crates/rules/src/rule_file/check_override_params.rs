//! `check_override_params`: the types of the params each `[[override]]` gives a baseline rule.

use super::baseline_set::BaselineSet;
use super::check_param_types::check_param_types;
use super::param_type::ParamType;
use super::read_required_params::read_required_params;
use crate::Problem;
use toml::de::DeTable;

/// For each `[[override]]` whose `rule` is a string and whose `params` is a table, the params checked against the
/// required-param types of that rule in the first set of `taken` that holds it. Anything else is left to others:
/// read_overrides reports a malformed override, resolve_baseline an id no taken set holds, and load_set a set's own
/// problems (so a set that does not parse is skipped and its required_params problems are dropped here).
pub(crate) fn check_override_params(
    text: &str,
    root: &DeTable,
    taken: &[(&BaselineSet, usize)],
    out: &mut Vec<Problem>,
) {
    let Some(overrides) = root.get("override").and_then(|v| v.get_ref().as_array()) else {
        return;
    };
    for table in overrides.iter().filter_map(|item| item.get_ref().as_table()) {
        let id = table.get("rule").and_then(|v| v.get_ref().as_str());
        let params = table.get("params").and_then(|v| v.get_ref().as_table());
        let (Some(id), Some(params)) = (id, params) else {
            continue;
        };
        if let Some(types) = taken.iter().find_map(|(set, _)| set_types(set, id)) {
            check_param_types(text, id, &types, params, out);
        }
    }
}

/// The required params of rule `id` in `set`, with their types; None when the set does not parse or holds no such
/// rule.
fn set_types(set: &BaselineSet, id: &str) -> Option<Vec<(String, Option<ParamType>)>> {
    let doc = DeTable::parse(set.text).ok()?;
    let rules = doc.get_ref().get("rule")?.get_ref().as_array()?;
    let rule = rules
        .iter()
        .filter_map(|item| item.get_ref().as_table())
        .find(|rule| rule.get("id").and_then(|v| v.get_ref().as_str()) == Some(id))?;
    Some(read_required_params(set.text, rule, &mut Vec::new()))
}

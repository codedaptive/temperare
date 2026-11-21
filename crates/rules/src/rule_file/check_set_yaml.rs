//! `check_set_yaml`: every ast-grep rule of a baseline set names a YAML file the set carries.

use super::baseline_set::BaselineSet;
use super::problem_at::problem_at;
use crate::{Engine, Problem};
use toml::de::DeTable;

/// A Problem at the `check` value of each `[[rule]]` in `root` (the set's own `text`) whose `engine` is ast-grep and
/// whose `check` names no YAML file `set` carries. A `rule`, `engine` or `check` of the wrong type, or an unknown
/// engine, gives nothing here: read_rules reports those.
pub(crate) fn check_set_yaml(text: &str, root: &DeTable, set: &BaselineSet, out: &mut Vec<Problem>) {
    let Some(tables) = root.get("rule").and_then(|value| value.get_ref().as_array()) else {
        return;
    };
    for table in tables.iter().filter_map(|item| item.get_ref().as_table()) {
        let ast_grep = table
            .get("engine")
            .and_then(|engine| engine.get_ref().as_str())
            .is_some_and(|name| Engine::from_name(name) == Some(Engine::AstGrep));
        let Some(check) = table.get("check").filter(|_| ast_grep) else {
            continue;
        };
        let Some(path) = check.get_ref().as_str().filter(|path| set.yaml_for(path).is_none()) else {
            continue;
        };
        let (name, version) = (set.name, set.version);
        out.push(problem_at(
            text,
            check.span().start,
            format!("check \"{path}\" names a YAML file that baseline set {name}@{version} does not carry"),
            &format!(
                "Put {path} in crates/rules/baseline/{name}/{version}/ and list it in the set's yaml in EMBEDDED_SETS, \
                 or correct check."
            ),
        ));
    }
}

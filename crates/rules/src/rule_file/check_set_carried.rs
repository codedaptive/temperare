//! `check_set_carried`: every file a baseline set carries is named by one of its rules.

use super::baseline_set::BaselineSet;
use super::problem_at::problem_at;
use crate::{Engine, Problem};
use std::collections::BTreeSet;
use toml::de::DeTable;

/// A Problem for each case file and then each YAML file `set` carries that no `[[rule]]` of `root` (the set's own
/// `text`) names, each in the order the set lists them. A case file is named by `cases/<id>/<kind>/<file>` for a rule
/// whose `kind` case (failing or passing) lists `<file>` in `files`; a YAML file by the `check` of an ast-grep rule.
/// The names come from the TOML itself, so a rule whose case has another Problem still names its files. Carried data
/// has no place in the set's text, so every Problem here is at byte 0, 1:1.
pub(crate) fn check_set_carried(text: &str, root: &DeTable, set: &BaselineSet, out: &mut Vec<Problem>) {
    let (cases, yaml) = named(root);
    let (name, version) = (set.name, set.version);
    let unnamed_cases = set.cases.iter().map(|(path, _)| *path).filter(|path| !cases.contains(*path));
    let unnamed_yaml = set.yaml.iter().map(|(path, _)| *path).filter(|path| !yaml.contains(path));
    let unnamed = unnamed_cases
        .map(|path| ("case", "cases", path))
        .chain(unnamed_yaml.map(|path| ("YAML", "yaml", path)));
    for (what, list, path) in unnamed {
        out.push(problem_at(
            text,
            0,
            format!("baseline set {name}@{version} carries {what} file \"{path}\" that no rule names"),
            &format!(
                "Remove it from the set's {list} in crates/rules/src/rule_file/{name}_{version}.rs and from \
                 crates/rules/baseline/{name}/{version}/, or name it in a rule."
            ),
        ));
    }
}

/// The case file paths and the YAML files the rules of `root` name. A `rule`, entry, id, case, `files` or `check` of
/// the wrong shape names nothing: read_rules and read_set_cases report those.
fn named<'a>(root: &'a DeTable<'a>) -> (BTreeSet<String>, BTreeSet<&'a str>) {
    let (mut cases, mut yaml) = (BTreeSet::new(), BTreeSet::new());
    let tables = root.get("rule").and_then(|value| value.get_ref().as_array());
    for table in tables.into_iter().flatten().filter_map(|item| item.get_ref().as_table()) {
        let text_of = |key: &str| table.get(key).and_then(|value| value.get_ref().as_str());
        if text_of("engine").and_then(Engine::from_name) == Some(Engine::AstGrep) {
            yaml.extend(text_of("check"));
        }
        let Some(id) = text_of("id") else {
            continue;
        };
        for kind in ["failing", "passing"] {
            let files = table
                .get(kind)
                .and_then(|case| case.get_ref().as_table())
                .and_then(|case| case.get("files"))
                .and_then(|files| files.get_ref().as_array());
            let listed = files.into_iter().flatten().filter_map(|file| file.get_ref().as_str());
            cases.extend(listed.map(|file| format!("cases/{id}/{kind}/{file}")));
        }
    }
    (cases, yaml)
}

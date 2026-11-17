//! `read_kits`: the `[kits]` of the rule file.

use super::check_globs::check_globs;
use super::check_keys::check_keys;
use super::string_list::string_list;
use super::wrong_type::wrong_type;
use crate::Problem;
use std::collections::BTreeMap;
use toml::de::DeTable;

/// Each kit under `[kits]` with the path globs of it that compile; absent is none. A kit with problems is still
/// defined, so a rule naming it is not also reported.
pub(crate) fn read_kits(text: &str, root: &DeTable, out: &mut Vec<Problem>) -> BTreeMap<String, Vec<String>> {
    let Some(value) = root.get("kits") else {
        return BTreeMap::new();
    };
    let Some(kits) = value.get_ref().as_table() else {
        out.push(wrong_type(text, "kits", value, "a table of kits"));
        return BTreeMap::new();
    };
    kits.iter()
        .map(|(name, kit)| {
            let paths = match kit.get_ref().as_table() {
                Some(table) => {
                    check_keys(text, table, &["paths"], &format!("kit \"{}\"", name.get_ref()), out);
                    check_globs(text, &string_list(text, table, "paths", out), out)
                }
                None => {
                    out.push(wrong_type(text, name.get_ref(), kit, "a table with `paths`"));
                    Vec::new()
                }
            };
            (name.get_ref().to_string(), paths)
        })
        .collect()
}

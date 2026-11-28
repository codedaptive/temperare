//! `read_kits`: the `[kits]` of the rule file.

use super::check_globs::check_globs;
use super::check_keys::check_keys;
use super::scope_or_problem::scope_or_problem;
use super::string_list::string_list;
use super::string_value::string_value;
use super::wrong_type::wrong_type;
use crate::{Kit, Problem, Scope};
use toml::Spanned;
use toml::de::{DeTable, DeValue};

const KIT_KEYS: [&str; 2] = ["paths", "owner"];

/// Each kit under `[kits]` in file order; absent is none. The TOML table hands its keys back in name order, so the
/// kits are put in the order their names stand in `text`. A kit with problems is still defined, so a rule naming it
/// is not also reported.
pub(crate) fn read_kits(text: &str, root: &DeTable, out: &mut Vec<Problem>) -> Vec<Kit> {
    let Some(value) = root.get("kits") else {
        return Vec::new();
    };
    let Some(kits) = value.get_ref().as_table() else {
        out.push(wrong_type(text, "kits", value, "a table of kits"));
        return Vec::new();
    };
    let mut in_file_order: Vec<_> = kits.iter().collect();
    in_file_order.sort_by_key(|(name, _)| name.span().start);
    in_file_order.into_iter().map(|(name, kit)| read_kit(text, name.get_ref(), name.span().start, kit, out)).collect()
}

/// The kit `name`, whose key starts at byte `at`, from `kit`, with the path globs of it that compile; a blank
/// `owner` counts as none. Each glob compiles alone (check_globs kept only those), but the set can still pass
/// globset's size limit together: then a Problem at the kit's key and no scope, so the kit holds nothing.
fn read_kit(text: &str, name: &str, at: usize, kit: &Spanned<DeValue>, out: &mut Vec<Problem>) -> Kit {
    let Some(table) = kit.get_ref().as_table() else {
        out.push(wrong_type(text, name, kit, "a table with `paths`"));
        return Kit { name: name.to_string(), paths: Vec::new(), owner: None, scope: None };
    };
    let kit_place = format!("kit \"{name}\"");
    check_keys(text, table, &KIT_KEYS, &kit_place, out);
    let paths = check_globs(text, &kit_place, &string_list(text, table, "paths", out), out);
    let owner = string_value(text, table, "owner", out).map(|(owner, _)| owner).filter(|o| !o.trim().is_empty());
    let scope = if paths.is_empty() {
        None
    } else {
        scope_or_problem(
            Scope::build(&paths, &[], None),
            text,
            at,
            |e| format!("{kit_place}: its paths do not compile together: {e}"),
            "Split this kit's paths into smaller kits.",
            out,
        )
    };
    Kit { name: name.to_string(), paths, owner, scope }
}

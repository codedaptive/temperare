//! `read_rule`: one `[[rule]]` table of the rule file.

use super::bool_value::bool_value;
use super::check_globs::check_globs;
use super::check_keys::check_keys;
use super::check_not_option::check_not_option;
use super::engine_value::engine_value;
use super::kind_value::kind_value;
use super::kit_union::kit_union;
use super::problem_at::problem_at;
use super::required_string::required_string;
use super::scope_or_problem::scope_or_problem;
use super::severity_value::severity_value;
use super::string_list::string_list;
use super::string_value::string_value;
use super::wrong_type::wrong_type;
use crate::{Kit, Problem, Rule, Scope};
use toml::de::DeTable;

const PLACE: &str = "[[rule]]";
/// The keys of a repository's own `[[rule]]`.
const OWN_KEYS: [&str; 15] = [
    "id", "engine", "kind", "check", "config", "paths", "exclude", "kits", "outside_kits", "severity", "fix", "params",
    "enforces", "leaves", "only_with_base",
];
/// The keys of a baseline set's `[[rule]]`: the own keys and `required_params`.
const SET_KEYS: [&str; 16] = [
    "id", "engine", "kind", "check", "config", "paths", "exclude", "kits", "outside_kits", "severity", "fix", "params",
    "enforces", "leaves", "only_with_base", "required_params",
];

/// The rule in `table`, whose `[[rule]]` header starts at byte `at`; `params` is its `params` as a plain table
/// (empty when absent); `from_set` is true when the table comes from an embedded baseline set, whose ids start
/// "tm-" and whose rules may declare `required_params`, and false for a repository's own file, whose ids may not
/// start "tm-". Every check runs; any Problem leaves the rule out (None), and its scope is compiled only when there
/// is none, excluding both `exclude` and the paths of the `outside_kits` kits.
pub(crate) fn read_rule(
    text: &str,
    at: usize,
    table: &DeTable,
    kits: &[Kit],
    params: toml::Table,
    from_set: bool,
    out: &mut Vec<Problem>,
) -> Option<Rule> {
    let before = out.len();
    check_keys(text, table, if from_set { &SET_KEYS } else { &OWN_KEYS }, PLACE, out);
    let id = required_string(text, table, "id", PLACE, at, out);
    if let Some((id, id_at)) = &id {
        check_id(text, id, *id_at, from_set, out);
    }
    let enforces = string_value(text, table, "enforces", out).map(|(s, _)| s);
    let leaves = string_value(text, table, "leaves", out).map(|(s, _)| s);
    let only_with_base = bool_value(text, table, "only_with_base", out).is_some_and(|(b, _)| b);
    // In a repository's own file `required_params` is an unknown key, reported by check_keys alone.
    let required_params = if from_set { string_list(text, table, "required_params", out) } else { Vec::new() };
    let engine = engine_value(text, table, at, out);
    let kind = kind_value(text, table, out);
    let check = required_string(text, table, "check", PLACE, at, out);
    let config = string_value(text, table, "config", out);
    let paths = check_globs(text, &string_list(text, table, "paths", out), out);
    let exclude = check_globs(text, &string_list(text, table, "exclude", out), out);
    let kit_names = string_list(text, table, "kits", out);
    let kit_paths = kit_union(text, &kit_names, kits, "kits", out);
    let outside_names = string_list(text, table, "outside_kits", out);
    let outside_paths = kit_union(text, &outside_names, kits, "outside_kits", out).unwrap_or_default();
    let severity = severity_value(text, table, out);
    let fix = required_string(text, table, "fix", PLACE, at, out);
    if let Some(value) = table.get("params").filter(|v| !v.get_ref().is_table()) {
        out.push(wrong_type(text, "params", value, "a table"));
    }
    check_not_option(text, table, out);
    let (Some((id, _)), Some(engine), Some(kind), Some((check, _)), Some(severity), Some((fix, _))) =
        (id, engine, kind, check, severity, fix)
    else {
        return None;
    };
    if out.len() > before {
        return None;
    }
    let built = Scope::build(
        &paths,
        &exclude.iter().chain(&outside_paths).cloned().collect::<Vec<_>>(),
        kit_paths.as_deref(),
    );
    let scope = scope_or_problem(built, text, at, |e| e, "Correct the rule's globs.", out)?;
    Some(Rule {
        id,
        engine,
        kind,
        check,
        config: config.map(|(config, _)| config),
        paths,
        exclude,
        kits: kit_names.into_iter().map(|(name, _)| name).collect(),
        outside_kits: outside_names.into_iter().map(|(name, _)| name).collect(),
        severity,
        fix,
        params,
        scope,
        enforces,
        leaves,
        only_with_base,
        required_params: required_params.into_iter().map(|(name, _)| name).collect(),
        not_applicable: None,
        embedded_text: None,
    })
}

/// A Problem at `at` when `id` is empty or holds anything but a-z, 0-9 and '-'; for a well-formed id, a Problem when
/// a baseline set's id (`from_set`) does not start "tm-" or a repository's own id does.
fn check_id(text: &str, id: &str, at: usize, from_set: bool, out: &mut Vec<Problem>) {
    let well_formed = !id.is_empty() && id.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-');
    let (what, fix) = match (well_formed, from_set, id.starts_with("tm-")) {
        (false, _, _) => (
            format!("malformed id \"{id}\": an id is one or more of a-z, 0-9 and '-'"),
            "Write the id in lower-case letters, digits and dashes, e.g. \"no-stored-bool\".",
        ),
        (true, false, true) => (
            format!("id \"{id}\" starts with \"tm-\", which is kept for baseline rules"),
            "Give the rule an id that does not start with \"tm-\", or change the baseline rule with an [[override]].",
        ),
        (true, true, false) => (
            format!("baseline rule id \"{id}\" does not start with \"tm-\""),
            "Start every baseline rule's id with \"tm-\".",
        ),
        _ => return,
    };
    out.push(problem_at(text, at, what, fix));
}

//! `read_rule`: one `[[rule]]` table of the rule file.

use super::check_globs::check_globs;
use super::check_keys::check_keys;
use super::kit_union::kit_union;
use super::problem_at::problem_at;
use super::required_string::required_string;
use super::string_list::string_list;
use super::string_value::string_value;
use super::wrong_type::wrong_type;
use crate::{Engine, Problem, Rule, Scope, Severity};
use std::collections::BTreeMap;
use toml::de::DeTable;

const PLACE: &str = "[[rule]]";
const RULE_KEYS: [&str; 10] = ["id", "engine", "check", "config", "paths", "exclude", "kits", "severity", "fix", "params"];

/// The rule in `table`, whose `[[rule]]` header starts at byte `at`; `params` is its `params` as a plain table
/// (empty when absent). Every check runs; any Problem leaves the rule out (None), and its scope is compiled only
/// when there is none.
pub(crate) fn read_rule(
    text: &str,
    at: usize,
    table: &DeTable,
    kits: &BTreeMap<String, Vec<String>>,
    params: toml::Table,
    out: &mut Vec<Problem>,
) -> Option<Rule> {
    let before = out.len();
    check_keys(text, table, &RULE_KEYS, PLACE, out);
    let id = required_string(text, table, "id", PLACE, at, out);
    if let Some((id, id_at)) = &id {
        check_id(text, id, *id_at, out);
    }
    let engine = engine_value(text, table, at, out);
    let check = required_string(text, table, "check", PLACE, at, out);
    let config = string_value(text, table, "config", out);
    let paths = check_globs(text, &string_list(text, table, "paths", out), out);
    let exclude = check_globs(text, &string_list(text, table, "exclude", out), out);
    let kit_names = string_list(text, table, "kits", out);
    let kit_paths = kit_union(text, &kit_names, kits, out);
    let severity = severity_value(text, table, out);
    let fix = required_string(text, table, "fix", PLACE, at, out);
    if let Some(value) = table.get("params").filter(|v| !v.get_ref().is_table()) {
        out.push(wrong_type(text, "params", value, "a table"));
    }
    let (Some((id, _)), Some(engine), Some((check, _)), Some(severity), Some((fix, _))) =
        (id, engine, check, severity, fix)
    else {
        return None;
    };
    if out.len() > before {
        return None;
    }
    let scope = match Scope::build(&paths, &exclude, kit_paths.as_deref()) {
        Ok(scope) => scope,
        Err(e) => {
            out.push(problem_at(text, at, e, "Correct the rule's globs."));
            return None;
        }
    };
    Some(Rule {
        id,
        engine,
        check,
        config: config.map(|(config, _)| config),
        paths,
        exclude,
        kits: kit_names.into_iter().map(|(name, _)| name).collect(),
        severity,
        fix,
        params,
        scope,
    })
}

/// A Problem at `at` when `id` is empty or holds anything but a-z, 0-9 and '-'.
fn check_id(text: &str, id: &str, at: usize, out: &mut Vec<Problem>) {
    let well_formed = !id.is_empty() && id.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-');
    if !well_formed {
        out.push(problem_at(
            text,
            at,
            format!("malformed id \"{id}\": an id is one or more of a-z, 0-9 and '-'"),
            "Write the id in lower-case letters, digits and dashes, e.g. \"no-stored-bool\".",
        ));
    }
}

/// The rule's `engine`, which must be one of the five engine names.
fn engine_value(text: &str, table: &DeTable, at: usize, out: &mut Vec<Problem>) -> Option<Engine> {
    let (name, name_at) = required_string(text, table, "engine", PLACE, at, out)?;
    let engine = Engine::from_name(&name);
    if engine.is_none() {
        out.push(problem_at(
            text,
            name_at,
            format!("unknown engine \"{name}\""),
            "Use one of: clippy, swiftlint, ruff, ast-grep, tessera.",
        ));
    }
    engine
}

/// The rule's `severity`: Error when absent; None (with a Problem) when it is not "error" or "warning".
fn severity_value(text: &str, table: &DeTable, out: &mut Vec<Problem>) -> Option<Severity> {
    if table.get("severity").is_none() {
        return Some(Severity::Error);
    }
    let (name, at) = string_value(text, table, "severity", out)?;
    let severity = Severity::from_name(&name);
    if severity.is_none() {
        out.push(problem_at(text, at, format!("unknown severity \"{name}\""), "Use \"error\" or \"warning\"."));
    }
    severity
}

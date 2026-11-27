//! ZUT for read_overrides.rs.

use super::override_entry::OverrideEntry;
use super::read_overrides::read_overrides;
use crate::{Problem, Severity};
use toml::de::DeTable;

const GLOB_FIX: &str = "Write a glob such as \"src/**/*.rs\": `*` stays within one folder, `**` crosses folders.";

fn read(text: &str) -> (Vec<OverrideEntry>, Vec<Problem>) {
    let doc = DeTable::parse(text).unwrap();
    let mut out = Vec::new();
    let entries = read_overrides(text, doc.get_ref(), &mut out);
    (entries, out)
}

fn problem(line: usize, col: usize, what: &str, fix: &str) -> Problem {
    Problem { line, col, what: what.to_string(), fix: fix.to_string() }
}

/// A table with one key besides `rule = "tm-x"` (whose string starts at byte 20) is read as `entry` with the one
/// Problem `expected`: a bad field is read as not given and the rest of the override is kept.
fn assert_kept_with(third_line: &str, entry: OverrideEntry, expected: Problem) {
    let text = format!("[[override]]\nrule = \"tm-x\"\n{third_line}\n");
    assert_eq!(read(&text), (vec![entry], vec![expected]));
}

fn only_rule(rule: &str, at: usize) -> OverrideEntry {
    OverrideEntry {
        rule: rule.to_string(),
        at,
        severity: None,
        off: false,
        paths: None,
        exclude: None,
        params: None,
    }
}

#[test]
fn no_override_key_is_no_entry_and_no_problem() {
    assert_eq!(read("[[rule]]\nid = \"a\"\n"), (vec![], vec![]));
}

#[test]
fn every_key_given_is_read_back_exactly() {
    // "[[override]]\n" is 13 bytes and "rule = " 7 more, so the rule string starts at byte 20.
    let text = "[[override]]\nrule = \"tm-x\"\nseverity = \"warning\"\noff = true\npaths = [\"src/**\"]\n\
                exclude = [\"src/gen/**\"]\nparams = { types = [\"User\", \"Order\"] }\n";
    let mut params = toml::Table::new();
    params.insert(
        "types".to_string(),
        toml::Value::Array(vec![toml::Value::String("User".to_string()), toml::Value::String("Order".to_string())]),
    );
    let expected = OverrideEntry {
        rule: "tm-x".to_string(),
        at: 20,
        severity: Some(Severity::Warning),
        off: true,
        paths: Some(vec!["src/**".to_string()]),
        exclude: Some(vec!["src/gen/**".to_string()]),
        params: Some(params),
    };
    assert_eq!(read(text), (vec![expected], vec![]));
}

#[test]
fn only_rule_given_leaves_every_other_field_none_or_false() {
    assert_eq!(read("[[override]]\nrule = \"tm-y\"\n"), (vec![only_rule("tm-y", 20)], vec![]));
}

#[test]
fn a_watch_severity_is_read() {
    let expected = OverrideEntry { severity: Some(Severity::Watch), ..only_rule("tm-x", 20) };
    assert_eq!(read("[[override]]\nrule = \"tm-x\"\nseverity = \"watch\"\n"), (vec![expected], vec![]));
}

#[test]
fn two_overrides_come_back_in_file_order() {
    // The second rule string: 13 + 14 ("rule = \"tm-b\"\n") + 11 ("off = true\n") + 13 + 7 = byte 58.
    let text = "[[override]]\nrule = \"tm-b\"\noff = true\n[[override]]\nrule = \"tm-a\"\n";
    let first = OverrideEntry { off: true, ..only_rule("tm-b", 20) };
    assert_eq!(read(text), (vec![first, only_rule("tm-a", 58)], vec![]));
}

#[test]
fn an_unknown_key_is_a_problem_at_the_key() {
    let fix = "Remove it, or use one of: rule, severity, off, paths, exclude, params.";
    assert_kept_with("level = 2", only_rule("tm-x", 20), problem(3, 1, "unknown key `level` in [[override]]", fix));
}

#[test]
fn a_missing_rule_is_a_problem_at_the_header() {
    let (entries, out) = read("[[override]]\noff = true\n");
    assert_eq!(entries, vec![]);
    let fix = "Add `rule = \"...\"` to this [[override]].";
    assert_eq!(out, vec![problem(1, 1, "[[override]] is missing `rule`", fix)]);
}

#[test]
fn a_rule_that_is_not_a_string_is_a_problem_at_the_value() {
    let (entries, out) = read("[[override]]\nrule = 7\n");
    assert_eq!(entries, vec![]);
    assert_eq!(out, vec![problem(2, 8, "`rule` must be a string, not integer", "Write `rule` as a string.")]);
}

#[test]
fn an_unknown_severity_is_a_problem_at_its_string() {
    let fix = "Use \"error\", \"warning\" or \"watch\".";
    assert_kept_with("severity = \"fatal\"", only_rule("tm-x", 20), problem(3, 12, "unknown severity \"fatal\"", fix));
}

#[test]
fn off_as_a_string_is_a_problem() {
    let what = "`off` must be a boolean, not string";
    assert_kept_with("off = \"yes\"", only_rule("tm-x", 20), problem(3, 7, what, "Write `off` as a boolean."));
}

#[test]
fn off_as_an_integer_is_a_problem() {
    let what = "`off` must be a boolean, not integer";
    assert_kept_with("off = 1", only_rule("tm-x", 20), problem(3, 7, what, "Write `off` as a boolean."));
}

#[test]
fn paths_not_an_array_is_a_problem() {
    let what = "`paths` must be an array of strings, not string";
    let expected = problem(3, 9, what, "Write `paths` as an array of strings.");
    assert_kept_with("paths = \"src\"", only_rule("tm-x", 20), expected);
}

#[test]
fn a_bad_exclude_glob_is_a_problem_at_its_string() {
    let what = "bad glob \"src/[a.rs\": error parsing glob 'src/[a.rs': unclosed character class; missing ']'";
    // The one glob did not compile, so the exclude list is given and empty.
    let entry = OverrideEntry { exclude: Some(vec![]), ..only_rule("tm-x", 20) };
    assert_kept_with("exclude = [\"src/[a.rs\"]", entry, problem(3, 12, what, GLOB_FIX));
}

#[test]
fn params_not_a_table_is_a_problem() {
    let what = "`params` must be a table, not integer";
    assert_kept_with("params = 3", only_rule("tm-x", 20), problem(3, 10, what, "Write `params` as a table."));
}

#[test]
fn a_second_override_for_one_rule_is_a_problem_and_left_out() {
    // The first override is kept as it is: the second one's `off = true` is not merged into it.
    let text = "[[override]]\nrule = \"tm-x\"\n[[override]]\nrule = \"tm-x\"\noff = true\n";
    let fix = "Put every change to one rule in a single [[override]].";
    let expected = problem(4, 8, "second [[override]] for \"tm-x\"", fix);
    assert_eq!(read(text), (vec![only_rule("tm-x", 20)], vec![expected]));
}

#[test]
fn override_not_an_array_is_a_problem() {
    let what = "`override` must be [[override]] tables, not integer";
    let expected = problem(1, 12, what, "Write `override` as [[override]] tables.");
    assert_eq!(read("override = 1\n"), (vec![], vec![expected]));
}

#[test]
fn an_element_that_is_not_a_table_is_a_problem_and_the_rest_are_read() {
    let what = "`override` must be an [[override]] table, not integer";
    let expected = problem(1, 13, what, "Write `override` as an [[override]] table.");
    let text = "override = [1, { rule = \"tm-z\" }]\n";
    // The inline table's rule string starts at byte 24.
    assert_eq!(read(text), (vec![only_rule("tm-z", 24)], vec![expected]));
}

#[test]
fn a_bad_severity_keeps_the_override_with_severity_not_given() {
    // The override is kept so that resolve_baseline can still check its rule id; the severity is read as not given.
    let text = "[[override]]\nrule = \"tm-x\"\nseverity = \"loud\"\n";
    let expected = problem(3, 12, "unknown severity \"loud\"", "Use \"error\", \"warning\" or \"watch\".");
    assert_eq!(read(text), (vec![only_rule("tm-x", 20)], vec![expected]));
}

#[test]
fn a_bad_field_in_a_second_override_still_leaves_it_out() {
    let text = "[[override]]\nrule = \"tm-x\"\n[[override]]\nrule = \"tm-x\"\noff = 1\n";
    let fix = "Put every change to one rule in a single [[override]].";
    let second = problem(4, 8, "second [[override]] for \"tm-x\"", fix);
    let off = problem(5, 7, "`off` must be a boolean, not integer", "Write `off` as a boolean.");
    assert_eq!(read(text), (vec![only_rule("tm-x", 20)], vec![second, off]));
}

#[test]
fn a_dashed_allow_value_is_a_problem_at_its_string() {
    // `params = { allow = [` is 20 characters, `"ok", ` 6 more: the "-bad" string starts at column 27.
    let what = "params.allow \"-bad\" begins with '-'";
    let fix = "A check, config or allow value is never an engine option: remove the leading '-'.";
    assert_kept_with("params = { allow = [\"ok\", \"-bad\"] }", only_rule("tm-x", 20), problem(3, 27, what, fix));
}

#[test]
fn severity_as_an_integer_is_a_problem() {
    let what = "`severity` must be a string, not integer";
    assert_kept_with("severity = 3", only_rule("tm-x", 20), problem(3, 12, what, "Write `severity` as a string."));
}

#[test]
fn exclude_not_an_array_is_a_problem() {
    let what = "`exclude` must be an array of strings, not integer";
    let expected = problem(3, 11, what, "Write `exclude` as an array of strings.");
    assert_kept_with("exclude = 3", only_rule("tm-x", 20), expected);
}

#[test]
fn a_paths_element_that_is_not_a_string_is_a_problem_and_the_strings_are_kept() {
    // `paths = ["src/**", ` is 19 characters, so the 3 is at column 20.
    let what = "`paths` must be an array of strings, not integer";
    let entry = OverrideEntry { paths: Some(vec!["src/**".to_string()]), ..only_rule("tm-x", 20) };
    assert_kept_with("paths = [\"src/**\", 3]", entry, problem(3, 20, what, "Write `paths` as an array of strings."));
}

#[test]
fn leaves_is_not_an_override_key() {
    let fix = "Remove it, or use one of: rule, severity, off, paths, exclude, params.";
    assert_kept_with("leaves = 3", only_rule("tm-x", 20), problem(3, 1, "unknown key `leaves` in [[override]]", fix));
}

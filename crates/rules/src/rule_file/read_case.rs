//! `read_case`: the failing or the passing case of one rule of a baseline set.

use super::baseline_set::BaselineSet;
use super::check_case_file_name::check_case_file_name;
use super::check_duplicate_case_file::check_duplicate_case_file;
use super::check_failing_line::check_failing_line;
use super::check_failing_path::check_failing_path;
use super::check_keys::check_keys;
use super::problem_at::problem_at;
use super::required_string::required_string;
use super::string_list::string_list;
use super::wrong_type::wrong_type;
use crate::{CaseFile, Problem};
use toml::Spanned;
use toml::de::{DeTable, DeValue};

const FAILING_KEYS: [&str; 3] = ["files", "path", "line"];
const PASSING_KEYS: [&str; 1] = ["files"];

/// The case `kind` ("failing" or "passing") of the set rule `id`, whose value in the set's `text` is `value`: its
/// files in the order `files` lists them, each with the text `set` carries at `cases/<id>/<kind>/<file>`; and for a
/// failing case the `path` and `line` of the finding it must give (None for a passing case), where `path` must be one
/// of the case's listed files and `line` a line of it. Every Problem goes to `out`, and any Problem gives None.
pub(crate) fn read_case(
    text: &str,
    value: &Spanned<DeValue>,
    id: &str,
    kind: &str,
    set: &BaselineSet,
    out: &mut Vec<Problem>,
) -> Option<(Vec<CaseFile>, Option<(String, usize)>)> {
    let Some(table) = value.get_ref().as_table() else {
        out.push(wrong_type(text, kind, value, "a table"));
        return None;
    };
    let before = out.len();
    let failing = kind == "failing";
    check_keys(text, table, if failing { &FAILING_KEYS } else { &PASSING_KEYS }, kind, out);
    let at = value.span().start;
    let (files, listed) = case_files(text, table, at, id, kind, set, out);
    let finding = if failing { finding(text, table, at, id, &listed, &files, out) } else { None };
    (out.len() == before).then_some((files, finding))
}

/// The case's carried files, and every file name `files` lists, each once. A `files` that is absent or an empty array
/// is a Problem at `at`, the case's value. A name listed again (the same after normalising, as `./a.rs` for `a.rs`),
/// or that starts with a separator or a drive letter or has a `..` part, is a Problem at its entry and is not looked
/// up; each other file the set does not carry is a Problem at its entry.
fn case_files(
    text: &str,
    table: &DeTable,
    at: usize,
    id: &str,
    kind: &str,
    set: &BaselineSet,
    out: &mut Vec<Problem>,
) -> (Vec<CaseFile>, Vec<String>) {
    let listed = table.get("files").map(|files| files.get_ref().as_array().is_none_or(|items| !items.is_empty()));
    if listed != Some(true) {
        out.push(problem_at(
            text,
            at,
            format!("the {kind} case of baseline rule \"{id}\" lists no files"),
            "List the case's files in files.",
        ));
        return (Vec::new(), Vec::new());
    }
    let (name, version) = (set.name, set.version);
    let (mut files, mut names) = (Vec::new(), Vec::new());
    for (file, file_at) in string_list(text, table, "files", out) {
        if !check_duplicate_case_file(text, &file, file_at, &names, id, kind, out) {
            continue;
        }
        names.push(file.clone());
        if !check_case_file_name(text, &file, file_at, id, kind, out) {
            continue;
        }
        let Some(carried) = set.case_for(&format!("cases/{id}/{kind}/{file}")) else {
            out.push(problem_at(
                text,
                file_at,
                format!("case file \"{file}\" of baseline rule \"{id}\" is not carried by baseline set {name}@{version}"),
                &format!(
                    "Put it at crates/rules/baseline/{name}/{version}/cases/{id}/{kind}/{file} and list it in the \
                     set's cases in crates/rules/src/rule_file/{name}_{version}.rs, or correct files."
                ),
            ));
            continue;
        };
        files.push(CaseFile { path: file, text: carried });
    }
    (files, names)
}

/// The failing case's `path` and `line`; each absent is a Problem at `at`, the case's value. A `path` that is not one
/// of `listed`, the case's file names, is a Problem at it; a `line` past the end of the carried file among `files`
/// at `path` is a Problem at the line.
fn finding(
    text: &str,
    table: &DeTable,
    at: usize,
    id: &str,
    listed: &[String],
    files: &[CaseFile],
    out: &mut Vec<Problem>,
) -> Option<(String, usize)> {
    let path = required_string(text, table, "path", "failing", at, out);
    if let Some((path, path_at)) = &path {
        check_failing_path(text, path, *path_at, listed, id, out);
    }
    let line = line(text, table, at, out);
    if let (Some((path, _)), Some((line, line_at))) = (&path, line) {
        check_failing_line(text, path, line, line_at, files, id, out);
    }
    Some((path?.0, line?.0))
}

/// The failing case's `line`, an integer of 1 or more, and its byte offset.
fn line(text: &str, table: &DeTable, at: usize, out: &mut Vec<Problem>) -> Option<(usize, usize)> {
    let Some(value) = table.get("line") else {
        out.push(problem_at(
            text,
            at,
            "failing is missing `line`".to_string(),
            "Add `line = N`, the line of the finding the failing case must give.",
        ));
        return None;
    };
    let line = value
        .get_ref()
        .as_integer()
        .and_then(|i| usize::from_str_radix(i.as_str(), i.radix()).ok())
        .filter(|line| *line >= 1);
    if line.is_none() {
        out.push(problem_at(
            text,
            value.span().start,
            format!("`line` must be a line number of 1 or more, not {}", text.get(value.span()).unwrap_or_default()),
            "Write the line of the finding the failing case must give, e.g. `line = 3`.",
        ));
    }
    line.map(|line| (line, value.span().start))
}

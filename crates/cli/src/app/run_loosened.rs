//! `run_loosened`: `tm rules --loosened --base <rev>`, from finding the rule file to what it loosened since `<rev>`.

use super::base_error::BaseError;
use super::discover_rule_file::discover_rule_file;
use super::read_base_rule_file::read_base_rule_file;
use super::write_failure::write_failure;
use super::write_problems::write_problems;
use super::RulesWorld;
use crate::{loosened_json, loosened_text, RulesArgs};
use serde_json::json;
use std::io::Write;
use std::path::Path;

/// Finds the rule file as `run_rules` does, reads it as committed at `rev` (the `--base`) and as it is now, and prints
/// what now loosened since the base (`rules::loosened`), as text ending in a `next:` line or as one JSON object
/// carrying `exit` and `next`. Exit 0 when nothing loosened, 1 when something did. A rule file that cannot be found and
/// a base that cannot be read are usage errors (exit 2, see `write_failure`); the base's problems, labelled
/// `<path> (at <rev>)`, and then now's, are exit 2 (see `write_problems`). Write errors are ignored: a closed pipe is
/// not tm's failure.
pub(crate) fn run_loosened(
    args: &RulesArgs,
    rev: &str,
    cwd: &Path,
    world: &RulesWorld,
    out: &mut dyn Write,
    err: &mut dyn Write,
) -> u8 {
    let path = match discover_rule_file(args.rules.as_deref(), cwd, &|| (world.toplevel)(cwd), world.exists) {
        Ok(path) => path,
        Err(e) => return write_failure(&e, args.json, out, err),
    };
    let base = match read_base_rule_file(&path, rev, world) {
        Ok(base) => base,
        Err(BaseError::Usage(e)) => return write_failure(&e, args.json, out, err),
        Err(BaseError::Problems(label, problems)) => return write_problems(&label, &problems, args.json, out),
    };
    let now = match (world.read)(&path) {
        Ok(now) => now,
        Err(problems) => return write_problems(&path, &problems, args.json, out),
    };
    write_list(&path, rev, &rules::loosened(&base, &now), args.json, out)
}

/// Prints `list` for the rule file at `path` against `rev`, as text then its `next:` line or, when `json`, as one
/// object with `exit` and `next` added; returns the exit, 0 when `list` is empty and 1 when it is not.
fn write_list(path: &Path, rev: &str, list: &[rules::Loosening], json: bool, out: &mut dyn Write) -> u8 {
    let exit = u8::from(!list.is_empty());
    let next = match (exit, json) {
        (0, _) => "nothing to do".to_string(),
        (_, true) => format!("have each loosening in loosened approved, or undo it in {}", path.display()),
        _ => format!("have each loosening above approved, or undo it in {}", path.display()),
    };
    if json {
        let mut value = loosened_json(path, rev, list);
        if let Some(object) = value.as_object_mut() {
            object.insert("exit".to_string(), json!(exit));
            object.insert("next".to_string(), json!(next));
        }
        let _ = writeln!(out, "{value}");
    } else {
        let _ = writeln!(out, "{}next: {next}", loosened_text(path, rev, list));
    }
    exit
}

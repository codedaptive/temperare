//! `run_rules`: `tm rules`, from finding the rule file to the listing and its exit code.

use super::discover_rule_file::discover_rule_file;
use super::write_failure::write_failure;
use super::write_problems::write_problems;
use super::RulesWorld;
use crate::{next_step, rules_json, rules_text, RulesArgs};
use serde_json::{json, Value};
use std::io::Write;
use std::path::Path;

/// Finds the rule file, reads it and prints its rules with each engine's availability (exit 0, whatever the
/// availability: listing is not checking) or its problems (exit 2), as text ending in a `next:` line or as one JSON
/// object carrying `exit` and `next`. A rule file that cannot be found is a usage error
/// on `err` (and, under `--json`, an `{"error", "exit", "next"}` object on `out`), exit 2. Write errors are
/// ignored: a closed pipe is not tm's failure.
pub fn run_rules(
    args: &RulesArgs,
    cwd: &Path,
    world: &RulesWorld,
    out: &mut dyn Write,
    err: &mut dyn Write,
) -> u8 {
    let path = match discover_rule_file(args.rules.as_deref(), cwd, &|| (world.toplevel)(cwd), world.exists) {
        Ok(path) => path,
        Err(e) => return write_failure(&e, args.json, out, err),
    };
    match (world.read)(&path) {
        Ok(rf) if args.json => write_json(out, rules_json(&path, &rf, world.availability), 0, None, &path),
        Ok(rf) => write_text(out, &rules_text(&path, &rf, world.availability), 0, None, &path),
        Err(problems) => write_problems(&path, &problems, args.json, out),
    }
}

/// `text`, then the `next:` line for `exit` and the rule file at `rule_file`; returns `exit`.
fn write_text(out: &mut dyn Write, text: &str, exit: u8, fix: Option<&str>, rule_file: &Path) -> u8 {
    let _ = writeln!(out, "{text}next: {}", next_step(exit, fix, rule_file, &[]));
    exit
}

/// The object with `exit` and `next` (for the rule file at `rule_file`) added, on one line; returns `exit`.
fn write_json(out: &mut dyn Write, mut value: Value, exit: u8, fix: Option<&str>, rule_file: &Path) -> u8 {
    if let Some(object) = value.as_object_mut() {
        object.insert("exit".to_string(), json!(exit));
        object.insert("next".to_string(), json!(next_step(exit, fix, rule_file, &[])));
    }
    let _ = writeln!(out, "{value}");
    exit
}

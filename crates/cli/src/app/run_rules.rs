//! `run_rules`: `tm rules`, from finding the rule file to the listing and its exit code, or with `--loosened` what the
//! rule file loosened since `--base`.

use super::discover_rule_file::discover_rule_file;
use super::rules_not_applicable::rules_not_applicable;
use super::run_loosened::run_loosened;
use super::write_failure::write_failure;
use super::write_problems::write_problems;
use super::RulesWorld;
use crate::{next_step, rules_json, rules_text, RulesArgs};
use serde_json::{json, Value};
use std::io::Write;
use std::path::Path;

/// With `--base`, runs `run_loosened` instead: what the rule file loosened since that base, exit 0, 1 or 2
/// (`parse_rules_args` gives `--base` only together with `--loosened`).
/// Otherwise finds the rule file, reads it and prints its rules with each engine's availability, the rules not
/// applicable here with the whys `tm check` run in `cwd` gives them (`world.no_file_rules`; no engine runs), and its
/// exceptions in force and expired on the UTC day holding `world.now` (exit 0, whatever the availability or the dates:
/// listing is not checking) or its problems (exit 2), as text ending in a `next:` line or as one JSON object carrying
/// `exit` and `next`. A rule file that cannot be found is a usage error on `err` (and, under `--json`, an `{"error",
/// "exit", "next"}` object on `out`), exit 2. Write errors are ignored: a closed pipe is not tm's failure.
pub fn run_rules(
    args: &RulesArgs,
    cwd: &Path,
    world: &RulesWorld,
    out: &mut dyn Write,
    err: &mut dyn Write,
) -> u8 {
    if let Some(rev) = args.base.as_deref() {
        return run_loosened(args, rev, cwd, world, out, err);
    }
    let path = match discover_rule_file(args.rules.as_deref(), cwd, &|| (world.toplevel)(cwd), world.exists) {
        Ok(path) => path,
        Err(e) => return write_failure(&e, args.json, out, err),
    };
    let day = report::utc_date((world.now)());
    let rf = match (world.read)(&path) {
        Ok(rf) => rf,
        Err(problems) => return write_problems(&path, &problems, args.json, out),
    };
    // `cwd` is the folder `tm check` hands the engines as `repo`, so both judge the same folder.
    let not_applicable = rules_not_applicable(&rf, &(world.no_file_rules)(&rf, cwd));
    if args.json {
        write_json(out, rules_json(&path, &rf, world.availability, day, &not_applicable), 0, None, &path)
    } else {
        write_text(out, &rules_text(&path, &rf, world.availability, day, &not_applicable), 0, None, &path)
    }
}

/// `text`, then the `next:` line for `exit` and the rule file at `rule_file`; returns `exit`.
fn write_text(out: &mut dyn Write, text: &str, exit: u8, fix: Option<&str>, rule_file: &Path) -> u8 {
    let _ = writeln!(out, "{text}next: {}", next_step(exit, fix, rule_file, &[], false));
    exit
}

/// The object with `exit` and `next` (for the rule file at `rule_file`) added, on one line; returns `exit`.
fn write_json(out: &mut dyn Write, mut value: Value, exit: u8, fix: Option<&str>, rule_file: &Path) -> u8 {
    if let Some(object) = value.as_object_mut() {
        object.insert("exit".to_string(), json!(exit));
        object.insert("next".to_string(), json!(next_step(exit, fix, rule_file, &[], true)));
    }
    let _ = writeln!(out, "{value}");
    exit
}

//! `run_check`: `tm check`, from finding the rule file to the report and its exit code.

use super::discover_rule_file::discover_rule_file;
use super::only_paths::only_paths;
use super::report_engines::report_engines;
use super::resolve_base::resolve_base;
use super::rule_counts::rule_counts;
use super::select_rules::select_rules;
use super::write_failure::write_failure;
use super::write_problems::write_problems;
use super::CheckWorld;
use crate::{
    append_savings_log, check_json, exit_code, failed_tails, next_step, saved_json, savings_from, savings_line_on,
    savings_log_line, tokens_saved_line, CheckArgs, RuleCount, Savings, UsageError,
};
use engines::NotAvailableCause;
use serde_json::Value;
use std::collections::HashMap;
use std::io::Write;
use std::path::Path;

/// Finds and reads the rule file, keeps the rules and not-applicable rules of `--kind`, turns the paths into paths
/// under the rule file's folder and resolves `--base` before any engine runs; then runs the rules, against the base
/// commit when `--base` is given, and prints the report, which carries only the engines found for the rule file's
/// folder and probed for a version, each with its version or none; the report is given each rule the engines found
/// no file for, as (rule id, the engines' why). As text it is report's text: the kept findings, one line per
/// not-available rule, one per failed rule, one per covers-nothing rule, one per expired exception, then the
/// `engines:` line listing those engines (absent when there are none), then the counts line, which ends with the
/// not-applicable, the covers-nothing and then the expired count (a not-applicable rule gets no line, only that
/// count); then each failed rule's stderr tail and a `next:` line. As JSON it is one object (the `engines` field)
/// carrying `exit` and `next` (the tails are its failed rules' `stderr_tail`). Either way it returns the report's exit
/// code. The text opens with the tokens-saved line when at least one engine run was counted and the switch under HOME
/// is on; the JSON object always carries `saved`. When at least one engine run was counted, text or JSON, the switch is
/// read, so a config it cannot use warns once on `err` (under `--json` the switch's value changes nothing), and one
/// line goes to the savings log under HOME, naming the rule file's folder as an absolute path. A rule file that cannot
/// be found, a path outside its folder, a base that does not resolve or a report that cannot be built is a usage error
/// on `err` (and, under `--json`, an `{"error", "exit", "next"}` object on `out`); a rule file with problems lists them
/// on `out`. Both exit 2, with no savings line and no log line. Write errors are ignored: a closed pipe is not tm's
/// failure.
pub fn run_check(args: &CheckArgs, cwd: &Path, world: &CheckWorld, out: &mut dyn Write, err: &mut dyn Write) -> u8 {
    let path = match discover_rule_file(args.rules.as_deref(), cwd, &|| (world.toplevel)(cwd), world.exists) {
        Ok(path) => path,
        Err(e) => return write_failure(&e, args.json, out, err),
    };
    let rf = match (world.read)(&path) {
        Ok(rf) => select_rules(&rf, args.kind),
        Err(problems) => return write_problems(&path, &problems, args.json, out),
    };
    let only = match only_paths(&args.paths, cwd, &rf.folder) {
        Ok(only) => only,
        Err(e) => return write_failure(&e, args.json, out, err),
    };
    let base = match resolve_base(args.base.as_deref(), &rf.folder, world) {
        Ok(base) => base,
        Err(e) => return write_failure(&e, args.json, out, err),
    };
    let checked = (world.run_rules)(&rf, cwd, &only, base.as_ref().map(|(_, oid)| oid.as_str()));
    let counts = rule_counts(&checked.runs);
    let causes: HashMap<String, NotAvailableCause> =
        checked.runs.iter().filter_map(|r| Some((r.rule.clone(), r.not_available_cause?))).collect();
    let no_files = checked.runs.iter().filter_map(|r| Some((r.rule.clone(), r.no_files.clone()?))).collect();
    let outcomes = checked.runs.into_iter().map(|run| (run.rule, run.outcome)).collect();
    let base_at = base.as_ref().map(|(top, oid)| (top.as_path(), oid));
    let mut report = match (world.build_report)(&rf, outcomes, base_at, no_files) {
        Ok(report) => report,
        Err(r) => {
            let e = UsageError {
                what: format!("cannot build the report: {r}"),
                fix: "Check that the rule file's folder is inside the repository and git runs here.".to_string(),
            };
            return write_failure(&e, args.json, out, err);
        }
    };
    let folder = cwd.join(&rf.folder);
    report.engines = report_engines(checked.engines, &|engine| (world.availability)(engine, &folder));
    let exit = exit_code(&report);
    let pairs: Vec<_> = report.not_available.iter().map(|na| (na, causes.get(&na.rule).copied())).collect();
    let next = next_step(exit, None, &path, &pairs, args.json);
    let home = (world.home)();
    let savings = if args.json {
        write_json(check_json(&report, exit, &next), &counts, home.as_deref(), out, err)
    } else {
        write_text(
            &format!("{}{}next: {next}\n", report::render_text(&report), failed_tails(&report)),
            &counts,
            home.as_deref(),
            out,
            err,
        )
    };
    if savings.engine_runs >= 1 {
        append_savings_log(
            home.as_deref(),
            &savings_log_line((world.now)(), &folder, savings.output_tokens, savings.baseline_tokens),
        );
    }
    exit
}

/// Prints `object` on one line with `saved` added, its output characters counted on the object as printed without
/// `saved` and without the newline; returns the savings. When an engine run was counted, the switch under `home` is
/// read for its warning on `err` alone, as text reads it: its value changes nothing in the object.
fn write_json(
    mut object: Value,
    counts: &[RuleCount],
    home: Option<&Path>,
    out: &mut dyn Write,
    err: &mut dyn Write,
) -> Savings {
    let savings = savings_from(counts, char_count(&object.to_string()));
    if savings.engine_runs >= 1 {
        savings_line_on(home, err);
    }
    if let Value::Object(map) = &mut object {
        map.insert("saved".to_string(), saved_json(counts, &savings));
    }
    let _ = writeln!(out, "{object}");
    savings
}

/// Prints `body`, opened by the savings line when an engine run was counted and the switch under `home` is on (the
/// switch is read only then); the output characters are `body`'s. Returns the savings.
fn write_text(
    body: &str,
    counts: &[RuleCount],
    home: Option<&Path>,
    out: &mut dyn Write,
    err: &mut dyn Write,
) -> Savings {
    let savings = savings_from(counts, char_count(body));
    if savings.engine_runs >= 1 && savings_line_on(home, err) {
        let _ = writeln!(out, "{}", tokens_saved_line(&savings));
    }
    let _ = out.write_all(body.as_bytes());
    savings
}

/// The characters (Unicode scalar values) of `text`.
fn char_count(text: &str) -> u64 {
    u64::try_from(text.chars().count()).unwrap_or(u64::MAX)
}

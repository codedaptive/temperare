//! `run_check`: `tm check`, from finding the rule file to the report and its exit code.

use super::discover_rule_file::discover_rule_file;
use super::only_paths::only_paths;
use super::resolve_base::resolve_base;
use super::select_rules::select_rules;
use super::write_failure::write_failure;
use super::write_problems::write_problems;
use super::CheckWorld;
use crate::{check_json, exit_code, failed_tails, next_step, CheckArgs, UsageError};
use std::io::Write;
use std::path::Path;

/// Finds and reads the rule file, keeps the rules of `--kind`, turns the paths into paths under the rule file's
/// folder and resolves `--base` before any engine runs; then runs the rules and prints the report as text, followed
/// by each failed rule's stderr tail and a `next:` line, or as one JSON object carrying `exit` and `next` (the tails
/// are its failed rules' `stderr_tail`), and returns the report's exit code. A rule file that cannot be found, a path
/// outside its folder, a base that does not resolve or a report that cannot be built is a usage error on `err` (and,
/// under `--json`, an `{"error", "exit", "next"}` object on `out`); a rule file with problems lists them on `out`.
/// Both exit 2. Write errors are ignored: a closed pipe is not tm's failure.
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
    let outcomes = (world.run_rules)(&rf, cwd, &only);
    let report = match (world.build_report)(&rf, outcomes, base.as_ref().map(|(top, oid)| (top.as_path(), oid))) {
        Ok(report) => report,
        Err(r) => {
            let e = UsageError {
                what: format!("cannot build the report: {r}"),
                fix: "Check that the rule file's folder is inside the repository and git runs here.".to_string(),
            };
            return write_failure(&e, args.json, out, err);
        }
    };
    let exit = exit_code(&report);
    let next = next_step(exit, None, &path, &report.not_available);
    if args.json {
        let _ = writeln!(out, "{}", check_json(&report, exit, &next));
    } else {
        let _ = writeln!(out, "{}{}next: {next}", report::render_text(&report), failed_tails(&report));
    }
    exit
}

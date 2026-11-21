//! `run_change_case`: one case of an only_with_base rule, run as a change against a base commit in a folder of its
//! own that is removed after it.

use super::judge_case::judge_case;
use super::write_case_files::write_case_files;
use super::BaselineWorld;
use crate::output::CaseVerdict;
use rules::{Rule, RuleCases, RuleFile, RuleOutcome};
use std::io::Write;
use std::path::Path;

/// Runs `rule` alone on its failing case (`failing`) or its passing case as a change against the case's base commit,
/// and judges what it gave against the expected finding (`judge_case`) after the base filter `tm check --base` applies:
/// only findings on lines added since the base are kept (`report::keep_added`). The failing case's base commits the
/// passing files, then the failing files are written over them; the passing case's base is the empty folder, then the
/// passing files are written. Files are written only inside the case folder (`write_case_files`). A file that cannot be
/// placed or written, a base that cannot be committed or added lines that cannot be read is broken, with its why. Once
/// the folder is made it is always removed, whatever the case came to; a folder that cannot be removed is a
/// `tm: cannot remove the case folder <folder>: <e>` line on `err` and leaves the verdict as it is. Write errors on
/// `err` are ignored.
pub(crate) fn run_change_case(
    rule: &Rule,
    failing: bool,
    cases: &RuleCases,
    world: &BaselineWorld,
    err: &mut dyn Write,
) -> CaseVerdict {
    let folder = match (world.make_folder)() {
        Ok(folder) => folder,
        Err(e) => return CaseVerdict::Broken { why: format!("cannot make the case folder: {e}") },
    };
    let verdict = match change_outcome(rule, failing, cases, &folder, world) {
        Ok(outcome) => judge_case(failing, &outcome, &cases.expected),
        Err(why) => CaseVerdict::Broken { why },
    };
    if let Err(e) = (world.remove_folder)(&folder) {
        let _ = writeln!(err, "tm: cannot remove the case folder {}: {e}", folder.display());
    }
    verdict
}

/// The case's base committed and its change written in `folder`, then `rule`'s outcome there run with that base, its
/// findings cut to the lines added since the base; or the why of the first step that failed.
fn change_outcome(
    rule: &Rule,
    failing: bool,
    cases: &RuleCases,
    folder: &Path,
    world: &BaselineWorld,
) -> Result<RuleOutcome, String> {
    let commit = || (world.commit_all)(folder).map_err(|e| format!("cannot commit the case's base: {e}"));
    let base = if failing {
        write_case_files(folder, &cases.passing, world)?;
        let base = commit()?;
        write_case_files(folder, &cases.failing, world)?;
        base
    } else {
        let base = commit()?;
        write_case_files(folder, &cases.passing, world)?;
        base
    };
    let rf = RuleFile { rules: vec![rule.clone()], ..RuleFile::new(folder.to_path_buf()) };
    let checked = (world.run_rules)(&rf, folder, &[], Some(&base));
    let Some(run) = checked.runs.into_iter().find(|run| run.rule == rule.id) else {
        return Err("the rule was not run".to_string());
    };
    match run.outcome {
        RuleOutcome::Findings(findings) => {
            let added = (world.added_since)(folder, &base)
                .map_err(|e| format!("cannot read the lines the case adds: {e}"))?;
            Ok(RuleOutcome::Findings(report::keep_added(findings, &added, &[], Path::new(""))))
        }
        other => Ok(other),
    }
}

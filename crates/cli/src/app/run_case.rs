//! `run_case`: one case of a baseline rule, run in a folder of its own that is removed after it.

use super::judge_case::judge_case;
use super::write_case_files::write_case_files;
use super::BaselineWorld;
use crate::output::CaseVerdict;
use rules::{CaseFile, ExpectedFinding, Rule, RuleFile};
use std::io::Write;

/// Runs `rule` alone on the case `files` (the failing case when `failing`, else the passing one) and judges what it
/// gave against `expected` (`judge_case`). The files are written into a new case folder, only inside it
/// (`write_case_files`): a name that is not a relative path inside it is broken and nothing is written for it. The
/// first file that cannot be placed or written stops the case before the rule runs. Once the folder is made it is
/// always removed, whatever the case came to; a folder that cannot be removed is a
/// `cctao: cannot remove the case folder <folder>: <e>` line on `err` and leaves the verdict as it is. Write errors on
/// `err` are ignored.
pub(crate) fn run_case(
    rule: &Rule,
    failing: bool,
    files: &[CaseFile],
    expected: &ExpectedFinding,
    world: &BaselineWorld,
    err: &mut dyn Write,
) -> CaseVerdict {
    let folder = match (world.make_folder)() {
        Ok(folder) => folder,
        Err(e) => return CaseVerdict::Broken { why: format!("cannot make the case folder: {e}") },
    };
    let verdict = match write_case_files(&folder, files, world) {
        Err(why) => CaseVerdict::Broken { why },
        Ok(()) => {
            let rf = RuleFile { rules: vec![rule.clone()], ..RuleFile::new(folder.clone()) };
            let checked = (world.run_rules)(&rf, &folder, &[], None);
            match checked.runs.iter().find(|run| run.rule == rule.id) {
                Some(run) => judge_case(failing, &run.outcome, run.not_available_cause, expected),
                None => CaseVerdict::Broken { why: "the rule was not run".to_string() },
            }
        }
    };
    if let Err(e) = (world.remove_folder)(&folder) {
        let _ = writeln!(err, "cctao: cannot remove the case folder {}: {e}", folder.display());
    }
    verdict
}

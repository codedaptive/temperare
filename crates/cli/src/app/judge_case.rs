//! `judge_case`: one case's verdict from what its rule's run came to.

use crate::output::CaseVerdict;
use super::case_rel_path::case_rel_path;
use engines::NotAvailableCause;
use rules::{ExpectedFinding, Finding, RuleOutcome};

/// The verdict of a failing case (`failing` true: the rule must give exactly `expected`) or of a passing case (the
/// rule must give no finding) from `outcome`. An engine not available is not available, with its why and `cause`; a failed run
/// is broken, naming its exit code (or that it had none) and the last non-blank line of its stderr when there is
/// one. The expected path is read by `case_rel_path`, the rule the case file was written by, so `./a.py`,
/// `src//./a.py` and `src/a.py` name the files `a.py` and `src/a.py`; an expected path `case_rel_path` refuses never
/// matches, and is printed as written.
pub(crate) fn judge_case(
    failing: bool,
    outcome: &RuleOutcome,
    cause: Option<NotAvailableCause>,
    expected: &ExpectedFinding,
) -> CaseVerdict {
    let findings = match outcome {
        RuleOutcome::NotAvailable { why, .. } => return CaseVerdict::NotAvailable { why: why.clone(), cause },
        RuleOutcome::Failed { code, stderr, .. } => return CaseVerdict::Broken { why: failed_why(*code, stderr) },
        RuleOutcome::Findings(findings) => findings,
    };
    let path = case_rel_path(&expected.path);
    let wanted = if failing {
        format!("{} at {}:{}", expected.rule_id, path.as_deref().unwrap_or(&expected.path), expected.line)
    } else {
        "no finding".to_string()
    };
    let holds = match findings.as_slice() {
        [] => !failing,
        [only] => failing && path.as_deref().is_ok_and(|path| is_expected(only, expected, path)),
        _ => false,
    };
    if holds {
        return CaseVerdict::Holds;
    }
    if findings.is_empty() {
        return CaseVerdict::Broken { why: format!("gave no finding; expected {wanted}") };
    }
    let gave: Vec<String> =
        findings.iter().map(|f| format!("{} at {}:{}", f.rule, f.path.as_str(), f.line)).collect();
    CaseVerdict::Broken { why: format!("gave {}; expected {wanted}", gave.join(", ")) }
}

/// Whether `finding` is the expected one: same rule, the same path as `path` (the expected path as `case_rel_path`
/// reads it), same line.
fn is_expected(finding: &Finding, expected: &ExpectedFinding, path: &str) -> bool {
    finding.rule == expected.rule_id
        && finding.path.as_str() == path
        && usize::try_from(finding.line) == Ok(expected.line)
}

/// `the rule failed, exit <code>` or `the rule failed, no exit code`, then `: <line>` for the last non-blank line of
/// `stderr` when there is one.
fn failed_why(code: Option<i32>, stderr: &str) -> String {
    let head = match code {
        Some(code) => format!("the rule failed, exit {code}"),
        None => "the rule failed, no exit code".to_string(),
    };
    match stderr.lines().rev().find(|line| !line.trim().is_empty()) {
        Some(line) => format!("{head}: {line}"),
        None => head,
    }
}

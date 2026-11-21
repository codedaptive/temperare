//! `render_text`: the report as the text a person reads.

use crate::{EngineVersion, Failed, NotApplicable, NotAvailable, Report};
use rules::{Finding, Kind, Severity};

/// The report as text: a three-line block and a blank line per kept finding, one line per not-available rule, one
/// line per failed rule, one line per not-applicable rule in the report's order, one line of the engines that ran and
/// their versions (none when the list is empty), then the counts line, ending with the not-applicable count.
/// Silenced findings are counted, not listed; a failed rule's stderr tail is printed by the caller (cli's
/// `failed_tails`), and `--json` carries it. Every line ends with '\n'.
pub fn render_text(report: &Report) -> String {
    let mut text = String::new();
    report.findings.iter().for_each(|finding| text.push_str(&finding_block(finding)));
    report.not_available.iter().for_each(|rule| text.push_str(&not_available_line(rule)));
    report.failed.iter().for_each(|rule| text.push_str(&failed_line(rule)));
    report.not_applicable.iter().for_each(|rule| text.push_str(&not_applicable_line(rule)));
    text.push_str(&engines_line(&report.engines));
    let counts = &report.counts;
    text.push_str(&format!(
        "{} findings ({} error, {} warning; {} security), {} silenced, {} exceptions in force, {} not available, {} failed, {} not applicable\n",
        counts.findings,
        counts.errors,
        counts.warnings,
        counts.security,
        counts.silenced,
        report.exceptions_in_force,
        counts.not_available,
        counts.failed,
        counts.not_applicable,
    ));
    text
}

/// A finding with no line prints line 0: `path:0`.
fn finding_block(finding: &Finding) -> String {
    let col = finding.col.map(|col| format!(":{col}")).unwrap_or_default();
    let owner = finding.owner.as_ref().map(|owner| format!("  owner {owner}")).unwrap_or_default();
    format!(
        "{}:{}{col}  {} {} {}{owner}\n  {}\n  fix: {}\n\n",
        finding.path.as_str(),
        finding.line().unwrap_or(0),
        severity_name(finding.severity),
        kind_name(finding.kind),
        finding.rule,
        finding.what,
        finding.fix,
    )
}

fn not_available_line(rule: &NotAvailable) -> String {
    format!("not available: {} ({}): {}\n", rule.rule, rule.engine.name(), rule.why)
}

fn not_applicable_line(rule: &NotApplicable) -> String {
    format!("not applicable: {} ({}): {}\n", rule.rule, rule.engine.name(), rule.why)
}

fn failed_line(rule: &Failed) -> String {
    let code = rule.code.map(|code| format!("exit {code}")).unwrap_or_else(|| "no exit code".to_string());
    format!("failed: {} ({}), {code}\n", rule.rule, rule.engine.name())
}

/// The engines in the report's order, joined by ", "; an empty list gives no line.
fn engines_line(engines: &[EngineVersion]) -> String {
    if engines.is_empty() {
        return String::new();
    }
    let named: Vec<String> = engines
        .iter()
        .map(|one| match &one.version {
            Some(version) => format!("{} {version}", one.engine.name()),
            None => format!("{} (version unknown)", one.engine.name()),
        })
        .collect();
    format!("engines: {}\n", named.join(", "))
}

/// Severity's and kind's rule-file names; `rules` reads them (`from_name`) and writes them through `Serialize`, but
/// has no `&str` name accessor for either.
fn severity_name(severity: Severity) -> &'static str {
    match severity {
        Severity::Error => "error",
        Severity::Warning => "warning",
    }
}

fn kind_name(kind: Kind) -> &'static str {
    match kind {
        Kind::Quality => "quality",
        Kind::Security => "security",
    }
}

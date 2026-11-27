//! `render_text`: the report as the text a person reads.

use crate::{severity_name, CoversNothing, EngineVersion, Expired, Failed, NotAvailable, Report};
use rules::{Finding, Kind};

/// The report as text: a three-line block and a blank line per kept finding, one line per not-available rule, one
/// line per failed rule, one line per covers-nothing rule, one line per expired exception (each list in the report's
/// order), one line of the engines that ran and their versions (none when the list is empty), then the counts line,
/// ending with the not-applicable, the covers-nothing and then the expired count, all printed even when 0.
/// A not-applicable rule gets no line of its own: it is counted in the counts line, and `--json` lists it.
/// Silenced findings are counted, not listed; a failed rule's stderr tail is printed by the caller (cli's
/// `failed_tails`), and `--json` carries it. Every line ends with '\n'.
pub fn render_text(report: &Report) -> String {
    let mut text = String::new();
    report.findings.iter().for_each(|finding| text.push_str(&finding_block(finding)));
    report.not_available.iter().for_each(|rule| text.push_str(&not_available_line(rule)));
    report.failed.iter().for_each(|rule| text.push_str(&failed_line(rule)));
    report.covers_nothing.iter().for_each(|rule| text.push_str(&covers_nothing_line(rule)));
    report.expired.iter().for_each(|exception| text.push_str(&expired_line(exception)));
    text.push_str(&engines_line(&report.engines));
    let counts = &report.counts;
    text.push_str(&format!(
        "{} findings ({} error, {} warning; {} security), {} silenced, {} exceptions in force, {} not available, {} failed, {} not applicable, {} covers nothing, {} expired\n",
        counts.findings,
        counts.errors,
        counts.warnings,
        counts.security,
        counts.silenced,
        report.exceptions_in_force,
        counts.not_available,
        counts.failed,
        counts.not_applicable,
        counts.covers_nothing,
        counts.expired,
    ));
    text
}

/// A finding with no line prints its path alone, with no line or column: a column without a line means nothing.
fn finding_block(finding: &Finding) -> String {
    let location = match finding.line() {
        Some(line) => {
            let col = finding.col.map(|col| format!(":{col}")).unwrap_or_default();
            format!("{}:{line}{col}", finding.path.as_str())
        }
        None => finding.path.as_str().to_string(),
    };
    let owner = finding.owner.as_ref().map(|owner| format!("  owner {owner}")).unwrap_or_default();
    format!(
        "{location}  {} {} {}{owner}\n  {}\n  fix: {}\n\n",
        &severity_name(finding.severity),
        kind_name(finding.kind),
        finding.rule,
        finding.what,
        finding.fix,
    )
}

fn not_available_line(rule: &NotAvailable) -> String {
    format!("not available: {} ({}): {}\n", rule.rule, rule.engine.name(), rule.why)
}

fn covers_nothing_line(rule: &CoversNothing) -> String {
    format!("covers nothing: {} ({})\n", rule.rule, rule.engine.name())
}

fn expired_line(exception: &Expired) -> String {
    format!("expired: {} {} (expired {})\n", exception.rule, exception.path.as_str(), exception.expires)
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

/// Kind's rule-file name; `rules` reads it (`from_name`) and writes it through `Serialize`, but has no `&str` name
/// accessor for it. Severity's name comes from `crate::severity_name`.
fn kind_name(kind: Kind) -> &'static str {
    match kind {
        Kind::Quality => "quality",
        Kind::Security => "security",
    }
}

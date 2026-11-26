//! `rules_text`: the `tm rules` listing as text.

use super::serde_name::serde_name;
use super::split_exceptions::split_exceptions;
use std::fmt::Write;
use std::path::Path;

/// The rule file's path, each rule with its engine, kind, severity, scope and whether `availability` says its engine
/// can be run for the rule file's folder (`available`, or `not available: <why>`), then each of `not_applicable` in
/// the given order as `<rule>  <engine>  <why>`, then the exceptions in force on `day`, then those expired by `day`
/// with their `expires` date, the rules and exceptions in file order and every line ending in a newline.
/// `rf.exceptions_expired(day)` alone decides which are expired. All four header lines print with their count, (0)
/// included.
pub fn rules_text(
    file: &Path,
    rf: &rules::RuleFile,
    availability: &dyn Fn(rules::Engine) -> engines::Availability,
    day: rules::Date,
    not_applicable: &[report::NotApplicable],
) -> String {
    let mut text = format!("rule file: {}\nrules ({}):\n", file.display(), rf.rules.len());
    for r in &rf.rules {
        let engine = serde_name(serde_json::to_value(r.engine));
        let kind = serde_name(serde_json::to_value(r.kind));
        let severity = serde_name(serde_json::to_value(r.severity));
        // Writing to a String cannot fail.
        let available = match availability(r.engine) {
            engines::Availability::Available => "available".to_string(),
            engines::Availability::NotAvailable { why } => format!("not available: {why}"),
        };
        let _ = writeln!(text, "  {}  {engine}  {kind}  {severity}  paths: {}  {available}", r.id, scope(r));
    }
    let _ = writeln!(text, "not applicable ({}):", not_applicable.len());
    for n in not_applicable {
        let _ = writeln!(text, "  {}  {}  {}", n.rule, serde_name(serde_json::to_value(n.engine)), n.why);
    }
    let (in_force, expired) = split_exceptions(rf, day);
    let _ = writeln!(text, "exceptions in force ({}):", in_force.len());
    for e in in_force {
        let _ = writeln!(text, "{}approved: {}  reason: {}", head(e), e.approved, e.reason);
    }
    let _ = writeln!(text, "exceptions expired ({}):", expired.len());
    for (e, expires) in expired {
        let _ = writeln!(text, "{}expires: {expires}  approved: {}  reason: {}", head(e), e.approved, e.reason);
    }
    text
}

/// An exception line's start: `  <rule>  <path>  `, then `what: <what>  ` when the exception names one.
fn head(e: &rules::Exception) -> String {
    let what = e.what.as_ref().map(|w| format!("what: {w}  ")).unwrap_or_default();
    format!("  {}  {}  {what}", e.rule, e.path.as_str())
}

/// The paths joined ", " (`all files` when none), then `  exclude: ...`, `  kits: ...` and `  outside kits: ...`, each
/// only when its list is not empty.
fn scope(r: &rules::Rule) -> String {
    let mut scope = if r.paths.is_empty() { "all files".to_string() } else { r.paths.join(", ") };
    if !r.exclude.is_empty() {
        scope.push_str(&format!("  exclude: {}", r.exclude.join(", ")));
    }
    if !r.kits.is_empty() {
        scope.push_str(&format!("  kits: {}", r.kits.join(", ")));
    }
    if !r.outside_kits.is_empty() {
        scope.push_str(&format!("  outside kits: {}", r.outside_kits.join(", ")));
    }
    scope
}

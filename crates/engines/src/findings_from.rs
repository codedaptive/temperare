//! `findings_from`: a rule's hits as findings.

use crate::types::Hit;
use rules::{Finding, RelPath, Rule, RuleFile};

/// One finding per hit, in hit order, for each hit that `rule` applies to and whose path equals an `only` entry or
/// lies under one (the entry followed by '/'); any path when `only` is empty. The rule id, engine, kind, severity and
/// fix come from `rule`; the owner from `file.owner_for` the hit's path; the path, line, column and message from the
/// hit.
pub(crate) fn findings_from(file: &RuleFile, rule: &Rule, hits: Vec<Hit>, only: &[RelPath]) -> Vec<Finding> {
    hits.into_iter()
        .filter(|hit| (only.is_empty() || only.iter().any(|entry| covers(entry, &hit.path))) && rule.applies(&hit.path))
        .map(|hit| Finding {
            rule: rule.id.clone(),
            engine: rule.engine,
            kind: rule.kind,
            severity: rule.severity,
            owner: file.owner_for(&hit.path).map(str::to_string),
            path: hit.path,
            line: hit.line,
            col: hit.col,
            what: hit.what,
            fix: rule.fix.clone(),
        })
        .collect()
}

/// Whether `path` is `entry` itself or lies under it; `srcx/a.rs` is not under `src`.
fn covers(entry: &RelPath, path: &RelPath) -> bool {
    path.as_str().strip_prefix(entry.as_str()).is_some_and(|rest| rest.is_empty() || rest.starts_with('/'))
}

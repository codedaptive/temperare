//! `split_by_exceptions`: findings an exception covers, apart from the rest.

use rules::{Exception, Finding, RuleFile};

/// Splits `findings` by `RuleFile::exception_for`: the first list holds every finding no exception covers, the second
/// every covered finding with the exception that matched it. Both lists keep the input order.
pub fn split_by_exceptions(rules: &RuleFile, findings: Vec<Finding>) -> (Vec<Finding>, Vec<(Finding, Exception)>) {
    let mut kept = Vec::new();
    let mut silenced = Vec::new();
    for finding in findings {
        match rules.exception_for(&finding) {
            Some(exception) => silenced.push((finding, exception.clone())),
            None => kept.push(finding),
        }
    }
    (kept, silenced)
}

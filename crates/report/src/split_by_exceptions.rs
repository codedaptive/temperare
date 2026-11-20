//! `split_by_exceptions`: findings an exception covers, apart from the rest.

use rules::{Date, Exception, Finding, RuleFile};

/// Splits `findings` by `RuleFile::exception_on` given `day`: the first list holds every finding it gives no exception,
/// the second every finding it covers with that exception. Both lists keep the input order.
pub fn split_by_exceptions(
    rules: &RuleFile,
    findings: Vec<Finding>,
    day: Date,
) -> (Vec<Finding>, Vec<(Finding, Exception)>) {
    let mut kept = Vec::new();
    let mut silenced = Vec::new();
    for finding in findings {
        match rules.exception_on(&finding, day) {
            Some(exception) => silenced.push((finding, exception.clone())),
            None => kept.push(finding),
        }
    }
    (kept, silenced)
}

//! `split_exceptions`: a rule file's exceptions in force and expired on a day.

use rules::{Date, Exception, RuleFile};

/// The exceptions of `rf` in force on `day`, then those expired by it with their `expires` date, each list in file
/// order; every exception is in exactly one list. `rf.exceptions_expired(day)` alone decides which are expired.
pub(crate) fn split_exceptions(rf: &RuleFile, day: Date) -> (Vec<&Exception>, Vec<(&Exception, Date)>) {
    let judged = rf.exceptions_expired(day);
    let mut in_force = Vec::new();
    let mut expired = Vec::new();
    for e in &rf.exceptions {
        match e.expires {
            Some(date) if judged.iter().any(|x| std::ptr::eq(*x, e)) => expired.push((e, date)),
            _ => in_force.push(e),
        }
    }
    (in_force, expired)
}

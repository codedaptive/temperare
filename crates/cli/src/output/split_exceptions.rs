//! `split_exceptions`: a rule file's exceptions in force and expired on a day.

use rules::{Date, Exception, RuleFile};

/// The exceptions of `rf` in force on `day`, then those expired by it, each list in file order; every exception is in
/// exactly one. `rf.exceptions_expired(day)` alone decides which are expired.
pub(crate) fn split_exceptions(rf: &RuleFile, day: Date) -> (Vec<&Exception>, Vec<&Exception>) {
    let expired = rf.exceptions_expired(day);
    let in_force = rf.exceptions.iter().filter(|e| !expired.iter().any(|x| std::ptr::eq(*x, *e))).collect();
    (in_force, expired)
}

//! `CANNOT_TURN_OFF`: the end of the stop message for a dart, actionlint or hadolint setting that hides or governs
//! findings and that tm cannot turn off. It has no unit-test sibling: it has no behaviour, and the tests of its four
//! messages (dart, actionlint, hadolint) pin it.

/// Why a rule stops when an engine setting that hides or governs findings is one tm cannot override: the rule file is
/// the only home for an exception, so the message points there.
pub(crate) const CANNOT_TURN_OFF: &str = "tm cannot turn it off; put exceptions in the rule file";

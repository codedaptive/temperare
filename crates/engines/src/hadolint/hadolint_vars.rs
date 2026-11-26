//! `hadolint_vars`: the `HADOLINT_*` variable names among a list of names.

use std::ffi::OsString;

/// The names in `names` that start with `HADOLINT_` (case-sensitive), in the order given. hadolint reads options from
/// variables with that prefix (`HADOLINT_IGNORE` adds ignores outside the rule file), and a caller's environment must
/// not change a rule's answer, so the run removes every one of them, known or not. procio removes exact names only,
/// so the caller passes the names of tm's environment and removes these. The prefix is compared on the name's bytes,
/// so a name that is not UTF-8 is compared like any other, without a panic.
pub(crate) fn hadolint_vars(names: impl IntoIterator<Item = OsString>) -> Vec<OsString> {
    names.into_iter().filter(|name| name.as_encoded_bytes().starts_with(b"HADOLINT_")).collect()
}

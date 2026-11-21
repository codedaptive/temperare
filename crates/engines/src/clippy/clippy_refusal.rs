//! `clippy_refusal`: the clippy checks tm refuses before any flow.

/// The refusal of a `rustdoc::` check: a real lint, but clippy never runs rustdoc lints, so no clippy run could report
/// it and the rule would always read clean. `None` for any other check.
pub(crate) fn clippy_refusal(check: &str) -> Option<String> {
    check
        .starts_with("rustdoc::")
        .then(|| format!("clippy check `{check}` is a rustdoc lint: clippy never runs rustdoc lints, so tm cannot report it"))
}

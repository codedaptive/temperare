//! `CountMode`: the ast-grep entry's count mode, as the rule's params set it.

/// Count mode is on (`count = "setup-vs-tests"`). `shared_setup` is the files exempt from reporting; None when the
/// rule gives no `shared_setup` or an empty list, so nothing is exempt (a `Scope` built from no globs matches every
/// file, which would exempt them all).
#[derive(Debug)]
pub(crate) struct CountMode {
    pub(crate) shared_setup: Option<rules::Scope>,
}

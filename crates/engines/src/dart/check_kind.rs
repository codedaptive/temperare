//! `CheckKind`: what kind of dart code a rule's check is. A bare enum with no behaviour of its own, so it has no
//! unit-test sibling: its test is kind_of_tests.rs, through `kind_of`, which makes it.

/// A dart rule's check, as the probe run tells it (`kind_of`): a `Lint`, which dart reports only where
/// analysis_options.yaml enables it, or a `Diagnostic`, which dart reports wherever it finds one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CheckKind {
    Lint,
    Diagnostic,
}

//! `Stop`: why a rule gives no findings.

/// A rule's run ended without findings. `NotAvailable` is never "clean": `why` says what is missing. `Failed` means
/// the engine ran but gave no answer; `code` is its exit code when it exited, `stderr` what explains the failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Stop {
    NotAvailable { why: String },
    Failed { code: Option<i32>, stderr: String },
}

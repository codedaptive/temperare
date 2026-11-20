//! `Stop`: why a rule gives no findings.

/// A rule's run ended without findings. `NotAvailable` is never "clean": `why` says what is missing. `Failed` means
/// the rule gave no answer: the engine ran and gave none, or the rule was refused before running; `code` is the exit
/// code when the engine exited, `stderr` what explains the failure.
/// `UnknownCheck` means the engine does not know the rule's check (an engine too old for it); `said` is the engine's
/// own words, or for clippy tm's words for a name its help does not list. The rule is then not available, never clean.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Stop {
    NotAvailable { why: String },
    Failed { code: Option<i32>, stderr: String },
    UnknownCheck { said: String },
}

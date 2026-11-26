//! `NotAvailableCause`: why a rule is not available, as a value its caller can match.

/// The cause of a rule's `RuleOutcome::NotAvailable`, beside its why text. `EngineNotOnPath`: a name its entry
/// `needs` is not found: the engine's own programs (such as clippy's `cargo-clippy` or `clippy-driver`) and, for a
/// Node engine (eslint, stylelint, markdownlint-cli2), `node`.
/// `HelperNotOnPath`: another program the rule runs is not on PATH (such as `cargo` under clippy, or git run to list
/// ShellCheck's scripts). `UnknownCheck`: the installed engine said it does not know the rule's check.
/// `NoCargoProject`: a clippy rule whose rule file has no `Cargo.toml` at or above its folder.
/// `Other`: any other not-available rule (an engine never available, no base where the check needs one, a check not
/// in temperare's list of its engine's codes, an engine older than the release temperare's list of its codes was
/// taken from).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotAvailableCause {
    EngineNotOnPath,
    HelperNotOnPath,
    UnknownCheck,
    NoCargoProject,
    Other,
}

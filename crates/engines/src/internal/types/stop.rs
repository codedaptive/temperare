//! `Stop`: why a rule gives no findings.

/// A rule's run ended without findings. `NotAvailable` is never "clean": `why` says what is missing. `NotOnPath` is
/// the not-available case where a program the rule needs or runs (its engine's own, a helper it needs, or git) is not
/// on PATH; `program` is that program's name, and its why is `not_on_path_why`'s. `Failed` means the rule
/// gave no answer: the engine ran and gave none, or the rule was refused before running; `code` is the exit code when
/// the engine exited, `stderr` what explains the failure.
/// `UnknownCheck` means the engine does not know the rule's check (an engine too old for it); `said` is the engine's
/// own words, or for clippy cctao's words for a name its help does not list. The rule is then not available, never clean.
/// `NotListed` means the engine's check is well formed but not in cctao's list of that engine's codes; `said` is
/// the why after `<engine> <version>: `. The rule is then not available, never clean.
/// `NoFiles` means the rule ran nothing because no file it checks exists; `why` says which case. It is neither clean
/// nor not available: the runner gives it no findings and sets the run's `no_files` flag to `why`.
/// `NoCargoProject` means a clippy rule's rule file has no `Cargo.toml` at or above its folder; `why` is the text the
/// run reads. The rule is then not available, never clean.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Stop {
    NotAvailable { why: String },
    NotOnPath { program: String },
    Failed { code: Option<i32>, stderr: String },
    UnknownCheck { said: String },
    NotListed { said: String },
    NoFiles { why: String },
    NoCargoProject { why: String },
}

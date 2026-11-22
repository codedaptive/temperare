//! The golangci-lint adapter: the golangci-lint entry (its `golangci-lint run` command, the checks it refuses,
//! golangci-lint's unknown-linter words), its flow that keeps Go local, and the reader of its JSON output.

mod golangci_lint_entry;
mod golangci_lint_refusal;
mod parse_golangci_lint;
mod run_golangci_lint;
mod unknown_linters;

#[cfg(test)]
mod golangci_lint_entry_tests;
#[cfg(test)]
mod golangci_lint_refusal_tests;
#[cfg(test)]
mod parse_golangci_lint_tests;
#[cfg(test)]
mod run_golangci_lint_tests;
#[cfg(test)]
mod unknown_linters_tests;

pub use golangci_lint_entry::GOLANGCI_LINT;
pub(crate) use golangci_lint_refusal::golangci_lint_refusal;
pub use parse_golangci_lint::parse_golangci_lint;
pub(crate) use run_golangci_lint::run_golangci_lint;
pub(crate) use unknown_linters::unknown_linters;

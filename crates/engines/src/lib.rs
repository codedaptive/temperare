//! engines: one adapter per engine: run it, read its output into findings; "not available" when it is absent. See the Temperare mission (Layout).

pub mod actionlint;
pub mod ast_grep;
pub mod cargo_audit;
pub mod clang_tidy;
pub mod clippy;
pub mod dart;
pub mod detekt;
pub mod entry;
pub mod eslint;
pub mod git;
pub mod gitleaks;
pub mod golangci_lint;
pub mod hadolint;
pub mod markdownlint_cli2;
pub mod mypy;
pub mod periphery;
pub mod phpstan;
pub mod pmd;
pub mod pylint;
pub mod rubocop;
pub mod ruff;
pub mod sarif;
pub mod shellcheck;
pub mod sqlfluff;
pub mod stylelint;
pub mod swiftlint;
pub mod tessera;
pub mod vale;
pub mod vulture;
pub mod yamllint;

mod internal;

pub use availability::availability;
pub use availability_in::availability_in;
pub use entry::{entry_cmd, Entry, ParamArg, Params, Pick, ReadFn, TempFile, VersionProbe};
pub use entry_for::entry_for;
pub use entry_of::entry_of;
pub use no_file_rules::no_file_rules;
pub use run_rules_versioned_at::run_rules_versioned_at;
pub use types::{Availability, CheckedRules, EngineVersion, Hit, NoFileRule, NotAvailableCause, ParamError, ParseError, RuleRun};
pub(crate) use internal::*;

//! `engine_hits`: one rule's hits from its engine's entry, ShellCheck's scripts, clang-tidy's sources, PMD's Java
//! files, mypy's Python files, detekt's Kotlin sources, dart's .dart files, PHPStan's PHP files, actionlint's
//! workflows, sqlfluff's SQL files, hadolint's Dockerfiles, yamllint's YAML files, Vale's files, stylelint's CSS
//! files and markdownlint's Markdown files listed from the repository.

use crate::engine_hits_with::engine_hits_with;
use crate::host::Host;
use crate::shellcheck::shell_files;
use crate::types::{Hit, Stop};
use rules::Rule;
use std::path::Path;

/// `engine_hits_with` listing ShellCheck's scripts with `shell_files` (git): a ShellCheck rule's scripts, a cargo-audit
/// rule's Cargo.lock, a clang-tidy rule's C-family sources (`source_files` with `is_c_source`, git), a PMD rule's Java
/// files (`java_files`, git), a mypy rule's Python files (`python_files`, git), a detekt rule's Kotlin sources
/// (`kotlin_files`, git), a dart rule's .dart files (`dart_files`, git), a phpstan rule's PHP files (`source_files`
/// with `is_php_source`, git), an actionlint rule's workflows (`source_files` with `is_workflow`, git), a sqlfluff
/// rule's SQL files (`source_files` with `is_sql_source`, git), a hadolint rule's Dockerfiles (`source_files` with
/// `is_dockerfile`, git), a yamllint rule's YAML files (`source_files` with `is_yaml_source`, git), a vale rule's
/// files (`source_files` with `is_vale_source`, git), a stylelint rule's CSS files (`source_files` with
/// `is_css_source`, git) and a markdownlint rule's Markdown files (`source_files` with `is_markdown_source`, git) are
/// looked for before availability, so an absent engine with no file in the rule's paths is `NoFiles`, not
/// `NotAvailable` (Orc #1476: no file before not available). Then a rule with `only_with_base` and no base is
/// `NotAvailable` with `rules::ONLY_WITH_BASE_WHY` before any lookup. Otherwise the engine is looked up (no entry is
/// `NotAvailable`), then one whose program is missing is `NotOnPath`, and its flow never runs; an available engine
/// with an unresolved folder gets that folder's `Stop`; otherwise its entry's flow runs the rule through `host`.
pub(crate) fn engine_hits(rule: &Rule, folder: Result<&Path, &Stop>, host: &Host) -> Result<Vec<Hit>, Stop> {
    engine_hits_with(rule, folder, host, &shell_files)
}

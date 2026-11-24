//! `no_file_rules_listed`: the rules of a rule file settled as no-file before any engine runs, given the folder's
//! listings.

use crate::actionlint::is_workflow;
use crate::cargo_audit::{has_lock, lock_listed};
use crate::clang_tidy::is_c_source;
use crate::dart::dart_files;
use crate::detekt::kotlin_files;
use crate::engine_listed::engine_listed;
use crate::hadolint::is_dockerfile;
use crate::markdownlint_cli2::is_markdown_source;
use crate::mypy::python_files;
use crate::no_file_match::{no_file_match, NO_FILE_MATCH_WHY};
use crate::phpstan::is_php_source;
use crate::pmd::java_files;
use crate::refused::refused;
use crate::source_files::source_files;
use crate::sqlfluff::is_sql_source;
use crate::stylelint::is_css_source;
use crate::types::{NoFileRule, Stop};
use crate::vale::is_vale_source;
use crate::yamllint::is_yaml_source;
use rules::{Engine, RelPath, RuleFile};
use std::path::{Path, PathBuf};

/// The rules of `file`, in rule-file order, that are settled as no-file, each with its engine and why. Each rule is
/// settled in tm check's order, the first that applies deciding it: a rule `refused` refuses is left out; with a
/// `files` listing, a rule `no_file_match` says matches none of the files, with `NO_FILE_MATCH_WHY`; then a rule given
/// `Stop::NoFiles`, with that why, by `engine_listed` for ShellCheck (none of the scripts `scripts` lists in `folder`
/// is one it applies to), by `lock_listed` (a cargo-audit rule with a `folder` holding no Cargo.lock, or whose paths
/// do not cover it), then by `engine_listed` for clang-tidy, PMD, mypy, detekt, dart, phpstan, actionlint, sqlfluff,
/// hadolint, yamllint, vale, stylelint and markdownlint (none of the files its engine's lister finds in `folder` is
/// one it applies to). These are `run_rules_listed`'s step 2 and `engine_hits_with`'s step 1, through the same
/// functions. With no `files` listing no rule is settled by its paths, with no `folder` none by its Cargo.lock, and
/// with no `folder`, or a failed listing, none by its listed files. `only_with_base` plays no part: tm check settles a
/// rule as no-file before its step 1b looks at the base. Runs no engine and looks nothing up on PATH; the calls out
/// are `scripts`, for cargo-audit `has_lock`, which reads the disk, and the listers, which run git: for clang-tidy
/// `source_files` with `is_c_source`, for PMD `java_files`, for mypy `python_files`, for detekt `kotlin_files`, for
/// dart `dart_files`, and `source_files` with `is_php_source` for phpstan, `is_workflow` for actionlint,
/// `is_sql_source` for sqlfluff, `is_dockerfile` for hadolint, `is_yaml_source` for yamllint, `is_vale_source` for
/// vale, `is_css_source` for stylelint and `is_markdown_source` for markdownlint.
pub(crate) fn no_file_rules_listed(
    file: &RuleFile,
    folder: Option<&Path>,
    files: Option<&[RelPath]>,
    scripts: &dyn Fn(&Path) -> Result<Vec<PathBuf>, Stop>,
) -> Vec<NoFileRule> {
    file.rules
        .iter()
        .filter(|rule| refused(rule).is_none())
        .filter_map(|rule| {
            let why = if files.is_some_and(|files| no_file_match(rule, files)) {
                NO_FILE_MATCH_WHY.to_string()
            } else if let Some(Stop::NoFiles { why }) = engine_listed(Engine::ShellCheck, rule, folder, scripts) {
                why
            } else if let Some(Stop::NoFiles { why }) = lock_listed(rule, folder, &has_lock) {
                why
            } else if let Some(Stop::NoFiles { why }) = engine_listed(Engine::ClangTidy, rule, folder, &|folder: &Path| source_files(folder, is_c_source)) {
                why
            } else if let Some(Stop::NoFiles { why }) = engine_listed(Engine::Pmd, rule, folder, &java_files) {
                why
            } else if let Some(Stop::NoFiles { why }) = engine_listed(Engine::Mypy, rule, folder, &python_files) {
                why
            } else if let Some(Stop::NoFiles { why }) = engine_listed(Engine::Detekt, rule, folder, &kotlin_files) {
                why
            } else if let Some(Stop::NoFiles { why }) = engine_listed(Engine::Dart, rule, folder, &dart_files) {
                why
            } else if let Some(Stop::NoFiles { why }) = engine_listed(Engine::Phpstan, rule, folder, &|folder: &Path| source_files(folder, is_php_source)) {
                why
            } else if let Some(Stop::NoFiles { why }) = engine_listed(Engine::Actionlint, rule, folder, &|folder: &Path| source_files(folder, is_workflow)) {
                why
            } else if let Some(Stop::NoFiles { why }) = engine_listed(Engine::Sqlfluff, rule, folder, &|folder: &Path| source_files(folder, is_sql_source)) {
                why
            } else if let Some(Stop::NoFiles { why }) = engine_listed(Engine::Hadolint, rule, folder, &|folder: &Path| source_files(folder, is_dockerfile)) {
                why
            } else if let Some(Stop::NoFiles { why }) = engine_listed(Engine::Yamllint, rule, folder, &|folder: &Path| source_files(folder, is_yaml_source)) {
                why
            } else if let Some(Stop::NoFiles { why }) = engine_listed(Engine::Vale, rule, folder, &|folder: &Path| source_files(folder, is_vale_source)) {
                why
            } else if let Some(Stop::NoFiles { why }) = engine_listed(Engine::Stylelint, rule, folder, &|folder: &Path| source_files(folder, is_css_source)) {
                why
            } else if let Some(Stop::NoFiles { why }) = engine_listed(Engine::Markdownlint, rule, folder, &|folder: &Path| source_files(folder, is_markdown_source)) {
                why
            } else {
                return None;
            };
            Some(NoFileRule { rule: rule.id.clone(), engine: rule.engine, why })
        })
        .collect()
}

//! `engine_hits_with`: one rule's hits from its engine's entry, with ShellCheck's no-file outcome (through a given
//! lister of scripts), cargo-audit's, clang-tidy's, PMD's, mypy's, detekt's, dart's, PHPStan's, actionlint's,
//! sqlfluff's, hadolint's, yamllint's, Vale's, stylelint's and markdownlint's settled before availability, and a
//! rule that needs a base settled without one.

use crate::actionlint::is_workflow;
use crate::cargo_audit::{has_lock, lock_listed};
use crate::clang_tidy::is_c_source;
use crate::dart::dart_files;
use crate::detekt::kotlin_files;
use crate::engine_listed::engine_listed;
use crate::entry_for::entry_for;
use crate::hadolint::is_dockerfile;
use crate::host::Host;
use crate::markdownlint_cli2::is_markdown_source;
use crate::mypy::python_files;
use crate::no_adapter_why::no_adapter_why;
use crate::phpstan::is_php_source;
use crate::pmd::java_files;
use crate::source_files::source_files;
use crate::sqlfluff::is_sql_source;
use crate::stylelint::is_css_source;
use crate::types::{Hit, Stop};
use crate::unavailable::unavailable;
use crate::vale::is_vale_source;
use crate::yamllint::is_yaml_source;
use rules::{Engine, Rule, ONLY_WITH_BASE_WHY};
use std::path::{Path, PathBuf};

/// The hits of `rule` under `folder`: the rule file's canonical folder, or the `Stop` its resolving gave. In the
/// order a rule is settled (refusals, which the caller settles first; no file; not available; the run). Departures:
/// ESLint and RuboCop settle no file in their flow, after availability, since their config decides the files; ast-grep
/// has no no-file step of its own; a listing error in step 1 falls through to the steps below, where the engine's flow
/// lists again and stops with it (dart's flow lists nothing: its own `repo_top` call stops when git cannot give the
/// top):
/// 1. a rule with a resolved folder is `NoFiles` when `engine_listed`, with its engine's lister, finds no file the
///    rule applies to: ShellCheck with `scripts`; then (a cargo-audit rule when `lock_listed` finds no Cargo.lock
///    there, `has_lock` reading the disk, or the rule's paths do not cover it) clang-tidy with `source_files` and
///    `is_c_source`, PMD with `java_files`, mypy with `python_files`, detekt with `kotlin_files`, dart with
///    `dart_files`, and with `source_files` and `is_php_source` phpstan, `is_workflow` actionlint, `is_sql_source`
///    sqlfluff, `is_dockerfile` hadolint, `is_yaml_source` yamllint, `is_vale_source` vale, `is_css_source` stylelint
///    and `is_markdown_source` markdownlint, these listers running git. Nothing is looked up and the flow never runs;
/// 1b. a rule with `only_with_base` and no `host.base` is `NotAvailable` with `rules::ONLY_WITH_BASE_WHY`, whatever its
///    engine and whatever `folder` is: nothing is looked up on PATH and the flow never runs;
/// 2. an engine with no entry (`entry_for` gives `None`) is `NotAvailable` with `no_adapter_why`; an engine that
///    `unavailable` says cannot be run here gets that stop: `NotOnPath` for a program it needs not on PATH,
///    `NotAvailable` for one never available. Either way, whatever `folder` is, its flow never runs;
/// 3. an available engine with an unresolved folder gets that folder's `Stop`;
/// 4. otherwise its entry's flow runs it through `host`.
pub(crate) fn engine_hits_with(
    rule: &Rule,
    folder: Result<&Path, &Stop>,
    host: &Host,
    scripts: &dyn Fn(&Path) -> Result<Vec<PathBuf>, Stop>,
) -> Result<Vec<Hit>, Stop> {
    if let Some(stop) = engine_listed(Engine::ShellCheck, rule, folder.ok(), scripts) {
        return Err(stop);
    }
    if let Some(stop) = lock_listed(rule, folder.ok(), &has_lock) {
        return Err(stop);
    }
    if let Some(stop) = engine_listed(Engine::ClangTidy, rule, folder.ok(), &|folder: &Path| source_files(folder, is_c_source)) {
        return Err(stop);
    }
    if let Some(stop) = engine_listed(Engine::Pmd, rule, folder.ok(), &java_files) {
        return Err(stop);
    }
    if let Some(stop) = engine_listed(Engine::Mypy, rule, folder.ok(), &python_files) {
        return Err(stop);
    }
    if let Some(stop) = engine_listed(Engine::Detekt, rule, folder.ok(), &kotlin_files) {
        return Err(stop);
    }
    if let Some(stop) = engine_listed(Engine::Dart, rule, folder.ok(), &dart_files) {
        return Err(stop);
    }
    if let Some(stop) = engine_listed(Engine::Phpstan, rule, folder.ok(), &|folder: &Path| source_files(folder, is_php_source)) {
        return Err(stop);
    }
    if let Some(stop) = engine_listed(Engine::Actionlint, rule, folder.ok(), &|folder: &Path| source_files(folder, is_workflow)) {
        return Err(stop);
    }
    if let Some(stop) = engine_listed(Engine::Sqlfluff, rule, folder.ok(), &|folder: &Path| source_files(folder, is_sql_source)) {
        return Err(stop);
    }
    if let Some(stop) = engine_listed(Engine::Hadolint, rule, folder.ok(), &|folder: &Path| source_files(folder, is_dockerfile)) {
        return Err(stop);
    }
    if let Some(stop) = engine_listed(Engine::Yamllint, rule, folder.ok(), &|folder: &Path| source_files(folder, is_yaml_source)) {
        return Err(stop);
    }
    if let Some(stop) = engine_listed(Engine::Vale, rule, folder.ok(), &|folder: &Path| source_files(folder, is_vale_source)) {
        return Err(stop);
    }
    if let Some(stop) = engine_listed(Engine::Stylelint, rule, folder.ok(), &|folder: &Path| source_files(folder, is_css_source)) {
        return Err(stop);
    }
    if let Some(stop) = engine_listed(Engine::Markdownlint, rule, folder.ok(), &|folder: &Path| source_files(folder, is_markdown_source)) {
        return Err(stop);
    }
    if rule.only_with_base && host.base.is_none() {
        return Err(Stop::NotAvailable { why: ONLY_WITH_BASE_WHY.to_string() });
    }
    let Some(entry) = entry_for(rule.engine) else {
        return Err(Stop::NotAvailable { why: no_adapter_why(rule.engine) });
    };
    if let Some(stop) = unavailable(rule.engine, host.on_path) {
        return Err(stop);
    }
    let folder = folder.map_err(Stop::clone)?;
    (entry.run)(entry, rule, folder, host)
}

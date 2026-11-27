//! `no_file_listed`: a rule's no-file why from its engine's listing of its folder, the one ladder cctao check and the
//! no-file listing share.

use crate::actionlint::is_workflow;
use crate::cargo_audit::{has_lock, lock_listed};
use crate::clang_tidy::is_c_source;
use crate::dart::dart_files;
use crate::detekt::kotlin_files;
use crate::engine_listed::engine_listed;
use crate::hadolint::is_dockerfile;
use crate::markdownlint_cli2::is_markdown_source;
use crate::mypy::python_files;
use crate::phpstan::is_php_source;
use crate::pmd::java_files;
use crate::source_files::source_files;
use crate::sqlfluff::is_sql_source;
use crate::stylelint::is_css_source;
use crate::types::Stop;
use crate::vale::is_vale_source;
use crate::yamllint::is_yaml_source;
use rules::{Engine, Rule};
use std::path::{Path, PathBuf};

/// The why of the `Stop::NoFiles` the first step that applies gives `rule` in `folder`, else `None`. The steps, in
/// order: `engine_listed` for ShellCheck with `scripts`; `lock_listed` with `has_lock`, which reads the disk; then
/// `engine_listed` for clang-tidy (`source_files` with `is_c_source`), PMD (`java_files`), mypy (`python_files`),
/// detekt (`kotlin_files`), dart (`dart_files`), and with `source_files` phpstan (`is_php_source`), actionlint
/// (`is_workflow`), sqlfluff (`is_sql_source`), hadolint (`is_dockerfile`), yamllint (`is_yaml_source`), vale
/// (`is_vale_source`), stylelint (`is_css_source`) and markdownlint (`is_markdown_source`), these listers running
/// git. Only the step for the rule's own engine lists anything; with no `folder`, or a failed listing, the answer is
/// `None`. It is a `String`, not a `Stop`, because `no_script`, the only source of a step's answer, gives only
/// `NoFiles`.
pub(crate) fn no_file_listed(
    rule: &Rule,
    folder: Option<&Path>,
    scripts: &dyn Fn(&Path) -> Result<Vec<PathBuf>, Stop>,
) -> Option<String> {
    let listers: [(Engine, &dyn Fn(&Path) -> Result<Vec<PathBuf>, Stop>); 13] = [
        (Engine::ClangTidy, &|folder: &Path| source_files(folder, is_c_source)),
        (Engine::Pmd, &java_files),
        (Engine::Mypy, &python_files),
        (Engine::Detekt, &kotlin_files),
        (Engine::Dart, &dart_files),
        (Engine::Phpstan, &|folder: &Path| source_files(folder, is_php_source)),
        (Engine::Actionlint, &|folder: &Path| source_files(folder, is_workflow)),
        (Engine::Sqlfluff, &|folder: &Path| source_files(folder, is_sql_source)),
        (Engine::Hadolint, &|folder: &Path| source_files(folder, is_dockerfile)),
        (Engine::Yamllint, &|folder: &Path| source_files(folder, is_yaml_source)),
        (Engine::Vale, &|folder: &Path| source_files(folder, is_vale_source)),
        (Engine::Stylelint, &|folder: &Path| source_files(folder, is_css_source)),
        (Engine::Markdownlint, &|folder: &Path| source_files(folder, is_markdown_source)),
    ];
    let stop = engine_listed(Engine::ShellCheck, rule, folder, scripts)
        .or_else(|| lock_listed(rule, folder, &has_lock))
        .or_else(|| listers.iter().find_map(|(engine, lister)| engine_listed(*engine, rule, folder, *lister)))?;
    match stop {
        Stop::NoFiles { why } => Some(why),
        _ => None,
    }
}

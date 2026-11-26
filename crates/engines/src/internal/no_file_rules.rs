//! `no_file_rules`: the rules of a rule file that tm check settles as no-file before any engine runs. It runs git, so
//! its test is the zone integration test tests/no_file_rules.rs and it has no unit-test sibling; the pure part,
//! `no_file_rules_listed`, is tested in no_file_rules_listed_tests.rs.

use crate::files_in_folder::files_in_folder;
use crate::no_file_rules_listed::no_file_rules_listed;
use crate::rule_folder::rule_folder;
use crate::shellcheck::shell_files;
use crate::types::NoFileRule;
use rules::RuleFile;
use std::path::Path;

/// The rules of `file`, in rule-file order, that tm check (`run_rules_versioned_at` on the same `file` and `repo`)
/// settles as no-file before any engine runs, each with the why its `RuleRun::no_files` carries. A rule tm refuses
/// is left out; a rule with `only_with_base` is included as tm check includes it. The folder and its listings are
/// the ones tm check makes, so outside a git repository, or when the rule file's folder does not resolve, the list is
/// empty. Runs no engine; it runs git to list the folder's files, scripts, a clang-tidy rule's C-family sources, a
/// PMD rule's Java files, a mypy rule's Python sources, a detekt rule's Kotlin sources, a dart rule's .dart files, a
/// phpstan rule's PHP files, an actionlint rule's workflows, a sqlfluff rule's SQL files, a hadolint rule's
/// Dockerfiles, a yamllint rule's YAML files, a vale rule's files in a format Vale reads, a stylelint rule's CSS
/// files and a markdownlint rule's Markdown files, and reads the disk for a cargo-audit rule's Cargo.lock.
pub fn no_file_rules(file: &RuleFile, repo: &Path) -> Vec<NoFileRule> {
    let folder = rule_folder(file, repo).ok();
    let files = folder.as_deref().and_then(files_in_folder);
    no_file_rules_listed(file, folder.as_deref(), files.as_deref(), &shell_files)
}

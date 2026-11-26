//! `source_files`: the sources under a rule's folder that an engine is given, picked by a predicate. It runs git and
//! reads the real filesystem, so it has no unit-test sibling: the zone integration tests tests/clang_tidy.rs (kept by
//! `is_c_source`, through `run_clang_tidy` and `engine_listed`), tests/detekt.rs (through `kotlin_files`), tests/pmd.rs
//! (through `java_files`), tests/mypy.rs (through `python_files`), tests/dart.rs (through `dart_files`),
//! tests/phpstan.rs (kept by `is_php_source`, through `run_phpstan` and `engine_listed`), tests/actionlint.rs (kept by
//! `is_workflow`, through `run_actionlint` and `engine_listed`), tests/sqlfluff.rs (kept by `is_sql_source`,
//! through `run_sqlfluff` and `engine_listed`), tests/vale.rs (kept by `is_vale_source`, through `run_vale` and
//! `engine_listed`), tests/stylelint.rs (kept by `is_css_source`, through `run_stylelint` and `engine_listed`),
//! tests/markdownlint_cli2.rs (kept by `is_markdown_source`, through `run_markdownlint` and `engine_listed`),
//! tests/yamllint.rs (through `run_yamllint`), tests/hadolint.rs (through `run_hadolint`) and
//! integration/source_files_tests.rs cover it.

use crate::checked_out::checked_out;
use crate::folder_files::folder_files;
use crate::git_stop::git_stop;
use crate::is_regular_file::is_regular_file;
use crate::repo_top::repo_top;
use crate::types::Stop;
use ccvcslib::{GitVcs, Vcs};
use std::path::{Path, PathBuf};

/// The sources under `folder` (canonical), relative to it, in listing order: the tracked files, then the untracked
/// files git does not ignore, shaped by `folder_files` against `repo_top`. A file is kept when `keep` says so and
/// `is_regular_file` holds: a tracked folder replaced on disk by a symlink still lists its files as checked out, so
/// every component below the repository top is checked, and no path reached through a symlink, the file itself
/// included, reaches the engine. The listing runs git through ccvcslib, not the host, so it is not counted; a git
/// error is `git_stop`'s `Stop`.
pub(crate) fn source_files(folder: &Path, keep: fn(&Path) -> bool) -> Result<Vec<PathBuf>, Stop> {
    let top = repo_top(folder)?;
    let vcs = GitVcs::default();
    let tracked = checked_out(vcs.tracked_entries(&top).map_err(|err| git_stop(err, procio::which))?);
    let untracked = vcs.untracked_entries(&top).map_err(|err| git_stop(err, procio::which))?;
    Ok(folder_files(&top, folder, &tracked, &untracked)
        .into_iter()
        .map(|path| PathBuf::from(path.as_str()))
        .filter(|path| keep(path))
        .filter(|path| is_regular_file(&top, &folder.join(path)))
        .collect())
}

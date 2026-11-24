//! `source_files`: the sources under a rule's folder that an engine is given, picked by a predicate. It runs git and
//! reads the real filesystem, so it has no unit-test sibling: the zone integration tests tests/clang_tidy.rs (kept by
//! `is_c_source`, through `run_clang_tidy` and `engine_listed`), tests/detekt.rs (through `kotlin_files`), tests/pmd.rs
//! (through `java_files`), tests/mypy.rs (through `python_files`), tests/dart.rs (through `dart_files`),
//! tests/phpstan.rs (kept by `is_php_source`, through `run_phpstan` and `engine_listed`), tests/actionlint.rs (kept by
//! `is_workflow`, through `run_actionlint` and `engine_listed`), tests/sqlfluff.rs (kept by `is_sql_source`,
//! through `run_sqlfluff` and `engine_listed`), tests/vale.rs (kept by `is_vale_source`, through `run_vale` and
//! `engine_listed`), tests/stylelint.rs (kept by `is_css_source`, through `run_stylelint` and `engine_listed`) and
//! tests/markdownlint_cli2.rs (kept by `is_markdown_source`, through `run_markdownlint` and `engine_listed`) cover it.

use crate::folder_files::folder_files;
use crate::git_stop::git_stop;
use crate::repo_top::repo_top;
use crate::types::Stop;
use std::path::{Path, PathBuf};

/// The sources under `folder` (canonical), relative to it, in listing order: the tracked files, then the untracked
/// files git does not ignore, shaped by `folder_files` against `repo_top`. A file is kept when `keep` says so and it is
/// a regular file, not a symlink: git lists a symlinked folder as one entry, so only the last component can be a link,
/// and no link reaches the engine. The listing runs git through procio, not the host, so it is not counted; a git error
/// is `git_stop`'s `Stop`.
pub(crate) fn source_files(folder: &Path, keep: fn(&Path) -> bool) -> Result<Vec<PathBuf>, Stop> {
    let top = repo_top(folder)?;
    let tracked = procio::tracked_files::<&str>(folder, &[]).map_err(git_stop)?;
    let untracked = procio::untracked_files(folder).map_err(git_stop)?;
    Ok(folder_files(&top, folder, &tracked, &untracked)
        .into_iter()
        .map(|path| PathBuf::from(path.as_str()))
        .filter(|path| keep(path))
        .filter(|path| std::fs::symlink_metadata(folder.join(path)).is_ok_and(|meta| meta.file_type().is_file()))
        .collect())
}

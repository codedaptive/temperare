//! Unit tests for discover_rule_file.rs: each way to the rule file and each error, with git and the disk as closures.

use super::discover_rule_file;
use crate::UsageError;
use procio::GitError;
use std::path::{Path, PathBuf};

fn no_git() -> Result<PathBuf, GitError> {
    panic!("toplevel must not be called")
}

fn no_disk(_: &Path) -> bool {
    panic!("exists must not be called")
}

fn repo_top() -> Result<PathBuf, GitError> {
    Ok(PathBuf::from("/repo"))
}

#[test]
fn the_flag_is_joined_to_cwd_without_asking_git_or_the_disk() {
    let found = discover_rule_file(Some(Path::new("cfg/t.toml")), Path::new("/work"), &no_git, &no_disk);
    assert_eq!(found, Ok(PathBuf::from("/work/cfg/t.toml")));
}

#[test]
fn an_absolute_flag_is_taken_as_given() {
    let found = discover_rule_file(Some(Path::new("/etc/t.toml")), Path::new("/work"), &no_git, &no_disk);
    assert_eq!(found, Ok(PathBuf::from("/etc/t.toml")));
}

#[test]
fn the_file_at_the_repository_top_when_it_exists() {
    let exists = |p: &Path| p == Path::new("/repo/temperare.toml");
    let found = discover_rule_file(None, Path::new("/repo/src"), &repo_top, &exists);
    assert_eq!(found, Ok(PathBuf::from("/repo/temperare.toml")));
}

#[test]
fn no_file_at_the_repository_top() {
    let found = discover_rule_file(None, Path::new("/repo/src"), &repo_top, &|_: &Path| false);
    assert_eq!(
        found,
        Err(UsageError {
            what: "no rule file at /repo/temperare.toml".to_string(),
            fix: "Write temperare.toml at the top of the repository, or pass --rules <path>.".to_string(),
        })
    );
}

#[test]
fn outside_a_repository_names_cwd() {
    let toplevel = || Err(GitError::NotARepository { repo: PathBuf::from("/elsewhere") });
    let found = discover_rule_file(None, Path::new("/work"), &toplevel, &no_disk);
    assert_eq!(
        found,
        Err(UsageError {
            what: "not in a git repository: /work".to_string(),
            fix: "Run tm inside a git repository, or pass --rules <path>.".to_string(),
        })
    );
}

#[test]
fn any_other_git_error_is_quoted() {
    let toplevel =
        || Err(GitError::BadOutput { args: vec!["rev-parse".into(), "--show-toplevel".into()], stdout: vec![] });
    let found = discover_rule_file(None, Path::new("/work"), &toplevel, &no_disk);
    assert_eq!(
        found,
        Err(UsageError {
            what: "git could not find the repository top: git rev-parse --show-toplevel: unexpected output"
                .to_string(),
            fix: "Check that git runs here, or pass --rules <path>.".to_string(),
        })
    );
}

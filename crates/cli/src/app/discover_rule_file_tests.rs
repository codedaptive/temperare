//! ZUT for discover_rule_file.rs: each way to the rule file and each error, with git and the disk as closures.

use super::discover_rule_file;
use crate::UsageError;
use ccvcslib::{VcsError, VcsFailure};
use std::path::{Path, PathBuf};

fn no_git() -> Result<PathBuf, VcsError> {
    panic!("toplevel must not be called")
}

fn no_disk(_: &Path) -> bool {
    panic!("exists must not be called")
}

fn repo_top() -> Result<PathBuf, VcsError> {
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
    let exists = |p: &Path| p == Path::new("/repo/tao.toml");
    let found = discover_rule_file(None, Path::new("/repo/src"), &repo_top, &exists);
    assert_eq!(found, Ok(PathBuf::from("/repo/tao.toml")));
}

#[test]
fn no_tao_file_at_the_repository_top_though_temperare_toml_is_there() {
    let found = discover_rule_file(None, Path::new("/repo/src"), &repo_top, &|p: &Path| {
        p == Path::new("/repo/temperare.toml")
    });
    assert_eq!(
        found,
        Err(UsageError {
            what: format!("no rule file at {}", Path::new("/repo").join("tao.toml").display()),
            fix: "Write tao.toml at the top of the repository, or pass --rules <path>.".to_string(),
        })
    );
}

#[test]
fn outside_a_repository_names_cwd() {
    let toplevel = || Err(VcsError::NotAWorkingCopy);
    let found = discover_rule_file(None, Path::new("/work"), &toplevel, &no_disk);
    assert_eq!(
        found,
        Err(UsageError {
            what: "not in a git repository: /work".to_string(),
            fix: "Run cctao inside a git repository, or pass --rules <path>.".to_string(),
        })
    );
}

#[test]
fn any_other_git_error_is_quoted() {
    for (error, what) in [
        (VcsError::Unavailable, "git could not find the repository top: version control unavailable"),
        (
            VcsError::Failed(VcsFailure { message: "fatal: bad object".to_string(), exit_status: Some(128) }),
            "git could not find the repository top: fatal: bad object",
        ),
    ] {
        let toplevel = || Err(error.clone());
        let found = discover_rule_file(None, Path::new("/work"), &toplevel, &no_disk);
        assert_eq!(
            found,
            Err(UsageError {
                what: what.to_string(),
                fix: "Check that git runs here, or pass --rules <path>.".to_string(),
            }),
            "{error:?}"
        );
    }
}

//! Unit tests for resolve_base.rs, with closure worlds: no git, no disk, no PATH. A seam a test must not reach panics.

use super::resolve_base::resolve_base;
use super::CheckWorld;
use crate::UsageError;
use procio::{GitError, Oid};
use std::cell::RefCell;
use std::ffi::OsString;
use std::path::{Path, PathBuf};

const HEX: &str = "0123456789abcdef0123456789abcdef01234567";

/// A world whose `toplevel` and `rev_parse` are the given closures and whose other seams panic if called.
fn world<'a>(
    toplevel: &'a dyn Fn(&Path) -> Result<PathBuf, GitError>,
    rev_parse: &'a dyn Fn(&Path, &str) -> Result<Option<Oid>, GitError>,
) -> CheckWorld<'a> {
    CheckWorld {
        toplevel,
        exists: &|_| unreachable!("exists"),
        read: &|_| unreachable!("read"),
        rev_parse,
        run_rules: &|_, _, _| unreachable!("run_rules"),
        build_report: &|_, _, _| unreachable!("build_report"),
    }
}

#[test]
fn no_base_calls_nothing_and_is_none() {
    let w = world(&|_| unreachable!("toplevel"), &|_, _| unreachable!("rev_parse"));
    assert_eq!(resolve_base(None, Path::new("/repo/app"), &w), Ok(None));
}

#[test]
fn a_resolved_base_is_the_top_and_the_commit() {
    let asked = RefCell::new(Vec::new());
    let toplevel = |folder: &Path| {
        asked.borrow_mut().push(format!("toplevel {}", folder.display()));
        Ok(PathBuf::from("/repo"))
    };
    let rev_parse = |top: &Path, rev: &str| {
        asked.borrow_mut().push(format!("rev_parse {} {rev}", top.display()));
        Ok(Oid::parse(HEX))
    };
    let got = resolve_base(Some("main"), Path::new("/repo/app"), &world(&toplevel, &rev_parse));
    assert_eq!(got, Ok(Some((PathBuf::from("/repo"), Oid::parse(HEX).unwrap()))));
    assert_eq!(*asked.borrow(), vec!["toplevel /repo/app", "rev_parse /repo main"]);
}

#[test]
fn no_repository_is_a_usage_error() {
    let toplevel = |_: &Path| Err(GitError::NotARepository { repo: PathBuf::from("/tmp/x") });
    let got = resolve_base(Some("main"), Path::new("/tmp/x"), &world(&toplevel, &|_, _| unreachable!("rev_parse")));
    assert_eq!(
        got,
        Err(UsageError {
            what: "git could not find the repository top for --base: /tmp/x: not a git repository".to_string(),
            fix: "Run tm check inside a git repository to use --base, or leave --base out.".to_string(),
        })
    );
}

#[test]
fn a_rev_naming_no_commit_is_a_usage_error() {
    let got = resolve_base(
        Some("nosuchrev"),
        Path::new("/repo"),
        &world(&|_| Ok(PathBuf::from("/repo")), &|_, _| Ok(None)),
    );
    assert_eq!(
        got,
        Err(UsageError {
            what: "base 'nosuchrev' does not name a commit".to_string(),
            fix: "Pass a commit, branch or tag that exists, e.g. --base main.".to_string(),
        })
    );
}

#[test]
fn a_git_failure_resolving_the_rev_is_a_usage_error() {
    let rev_parse = |_: &Path, _: &str| {
        Err(GitError::Exit {
            args: vec![OsString::from("rev-parse"), OsString::from("main")],
            code: Some(128),
            stderr: b"fatal: broken\n".to_vec(),
        })
    };
    let got = resolve_base(Some("main"), Path::new("/repo"), &world(&|_| Ok(PathBuf::from("/repo")), &rev_parse));
    assert_eq!(
        got,
        Err(UsageError {
            what: "git could not resolve base 'main': git rev-parse main: exit 128: fatal: broken".to_string(),
            fix: "Check that git runs here and that main exists.".to_string(),
        })
    );
}

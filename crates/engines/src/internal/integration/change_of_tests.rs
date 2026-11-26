//! `change_of` against real throwaway git repositories, each removed when its test ends, pass or fail: the added
//! lines and whole untracked files under the folder, a bad base, the paths `keep` rejects, and untracked symlinks.

use crate::change_of::change_of;
use crate::types::Stop;
use rules::RelPath;
use std::fs;
use std::ops::Range;
use std::path::{Path, PathBuf};

#[path = "../../../tests/support/git.rs"]
mod git_support;

/// A throwaway repository, removed when dropped, so a failing assertion leaves nothing behind.
struct Repo(PathBuf);

impl Drop for Repo {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn repo(name: &str) -> Repo {
    let root = std::env::temp_dir().join(format!("engines-zit-{}-change-of-{name}", std::process::id()));
    fs::create_dir(&root).unwrap();
    let root = Repo(fs::canonicalize(root).unwrap());
    git_support::git(&root.0, &["init", "-q"], b"");
    root
}

/// Commits everything in `repo` and gives the commit.
fn commit_all(repo: &Path) -> String {
    git_support::git(repo, &["add", "-A"], b"");
    git_support::git(repo, &["-c", "user.name=Test", "-c", "user.email=test@example.invalid", "commit", "-q", "-m", "base"], b"");
    String::from_utf8(git_support::git(repo, &["rev-parse", "HEAD"], b"")).unwrap().trim().to_string()
}

fn files(given: &[(&str, &[Range<u32>])]) -> Vec<(RelPath, Vec<Range<u32>>)> {
    given.iter().map(|(path, ranges)| (RelPath::new(path), ranges.to_vec())).collect()
}

#[test]
fn the_change_under_the_folder_is_its_added_lines_and_its_whole_untracked_files() {
    let repo = repo("change");
    let root = &repo.0;
    fs::create_dir(root.join("sub")).unwrap();
    fs::write(root.join("sub/a.rs"), "one\ntwo\n").unwrap();
    fs::write(root.join("outside.rs"), "one\n").unwrap();
    let base = commit_all(root);
    fs::write(root.join("sub/a.rs"), "one\nnew1\ntwo\nnew2\n").unwrap();
    fs::write(root.join("outside.rs"), "one\nmore\n").unwrap();
    fs::write(root.join("outside_new.rs"), "x\n").unwrap();
    fs::write(root.join("sub/full.rs"), "a\nb\n").unwrap();
    fs::write(root.join("sub/open.rs"), "x\ny\nz").unwrap();
    fs::write(root.join("sub/empty.rs"), "").unwrap();
    fs::write(root.join("sub/bin.dat"), b"head\0tail\n").unwrap();
    let got = change_of(&root.join("sub"), &base, &|_| true);
    let want = files(&[("a.rs", &[2..3, 4..5]), ("empty.rs", &[]), ("full.rs", &[1..3]), ("open.rs", &[1..4])]);
    assert_eq!(got, Ok(want));
}

#[test]
fn a_base_that_names_no_commit_is_failed_with_no_code() {
    let repo = repo("bad-base");
    let root = &repo.0;
    fs::write(root.join("a.rs"), "one\n").unwrap();
    commit_all(root);
    let got = change_of(root, "no-such-base", &|_| true);
    assert_eq!(got, Err(Stop::Failed { code: None, stderr: "no-such-base: not a revision in this repository".into() }));
}

#[cfg(unix)]
#[test]
fn a_path_keep_rejects_is_absent_and_its_contents_are_never_read() {
    use std::os::unix::fs::PermissionsExt;
    let repo = repo("keep");
    let root = &repo.0;
    fs::write(root.join("a.rs"), "one\n").unwrap();
    fs::write(root.join("skip.rs"), "one\n").unwrap();
    let base = commit_all(root);
    fs::write(root.join("a.rs"), "one\ntwo\n").unwrap();
    fs::write(root.join("skip.rs"), "one\ntwo\n").unwrap();
    fs::write(root.join("new.rs"), "x\n").unwrap();
    let secret = root.join("secret.rs");
    fs::write(&secret, "hidden\n").unwrap();
    let readable = fs::metadata(&secret).unwrap().permissions();
    fs::set_permissions(&secret, fs::Permissions::from_mode(0o000)).unwrap();
    let rejected = [RelPath::new("skip.rs"), RelPath::new("secret.rs")];
    let got = change_of(root, &base, &|path| !rejected.contains(path));
    fs::set_permissions(&secret, readable).unwrap();
    assert_eq!(got, Ok(files(&[("a.rs", &[2..3]), ("new.rs", &[1..2])])));
}

#[cfg(unix)]
#[test]
fn an_untracked_symlink_and_what_lies_under_one_are_skipped() {
    use std::os::unix::fs::symlink;
    let repo = repo("symlink");
    let root = &repo.0;
    fs::write(root.join("a.rs"), "one\n").unwrap();
    let base = commit_all(root);
    fs::create_dir(root.join("real")).unwrap();
    fs::write(root.join("real/inner.rs"), "x\ny\n").unwrap();
    symlink("nowhere", root.join("dangling.rs")).unwrap();
    symlink("real/inner.rs", root.join("to_file.rs")).unwrap();
    symlink("real", root.join("linked")).unwrap();
    let got = change_of(root, &base, &|_| true);
    // Only the real file is read: neither link to it, nor whatever git lists for the linked folder.
    assert_eq!(got, Ok(files(&[("real/inner.rs", &[1..3])])));
}

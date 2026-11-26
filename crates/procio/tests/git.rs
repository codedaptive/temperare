//! procio's integration tests for the git layer: each operation on a throwaway repository the test makes
//! under the temp dir and removes. Setup runs git with the machine's global and system config shut out, so no
//! signing or default-branch setting can change what the repository holds.

use std::collections::BTreeMap;
use std::ffi::OsString;
use std::fs;
use std::ops::Range;
use std::path::{Path, PathBuf};
use std::process::Command;

use procio::{added_lines, blob_at, rev_parse, toplevel, tracked_files, untracked_files, GitError, Oid};

/// A scratch dir under the temp dir, unique to this process and test; removed when the guard drops.
struct Scratch {
    dir: PathBuf,
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.dir);
    }
}

fn scratch(test: &str) -> Scratch {
    let dir = std::env::temp_dir().join(format!("procio-zit-git-{}-{test}", std::process::id()));
    fs::create_dir(&dir).unwrap();
    Scratch { dir }
}

/// Runs one setup git command in `dir`, isolated from inherited repository variables and from machine config.
fn git(dir: &Path, args: &[&str]) -> Vec<u8> {
    let out = Command::new("git")
        .arg("-C")
        .arg(dir)
        .args(args)
        .env_remove("GIT_DIR")
        .env_remove("GIT_WORK_TREE")
        .env_remove("GIT_INDEX_FILE")
        .env_remove("GIT_COMMON_DIR")
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .output()
        .unwrap();
    assert!(out.status.success(), "setup git {args:?} failed: {}", String::from_utf8_lossy(&out.stderr));
    out.stdout
}

/// A scratch dir made into a repository on `main` with a local identity and signing off.
fn repo(test: &str) -> Scratch {
    let s = scratch(test);
    git(&s.dir, &["init", "-q", "-b", "main"]);
    git(&s.dir, &["config", "user.name", "procio zit"]);
    git(&s.dir, &["config", "user.email", "zit@procio.invalid"]);
    git(&s.dir, &["config", "commit.gpgsign", "false"]);
    s
}

fn write(dir: &Path, rel: &str, bytes: &[u8]) {
    let path = dir.join(rel);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, bytes).unwrap();
}

fn commit_all(dir: &Path, message: &str) {
    git(dir, &["add", "-A"]);
    git(dir, &["commit", "-q", "-m", message]);
}

fn head(dir: &Path) -> String {
    String::from_utf8(git(dir, &["rev-parse", "HEAD"])).unwrap().trim().to_string()
}

fn p(rel: &str) -> PathBuf {
    PathBuf::from(rel)
}

#[test]
fn toplevel_of_root_and_subdir_is_the_canonical_root() {
    let s = repo("toplevel_root_subdir");
    write(&s.dir, "sub/deeper/x.txt", b"x\n");
    let root = fs::canonicalize(&s.dir).unwrap();
    assert_eq!(toplevel(&s.dir).unwrap(), root);
    assert_eq!(toplevel(&s.dir.join("sub/deeper")).unwrap(), root);
}

#[test]
fn toplevel_of_a_plain_dir_is_not_a_repository() {
    let s = scratch("toplevel_plain_dir");
    match toplevel(&s.dir) {
        Err(GitError::NotARepository { repo }) => assert_eq!(repo, s.dir),
        other => panic!("expected NotARepository, got {other:?}"),
    }
}

#[test]
fn toplevel_of_a_missing_dir_is_exit() {
    let s = scratch("toplevel_missing_dir");
    match toplevel(&s.dir.join("never-made")) {
        Err(GitError::Exit { .. }) => {}
        other => panic!("expected Exit, got {other:?}"),
    }
}

#[test]
fn rev_parse_head_is_the_commit_oid() {
    let s = repo("rev_parse_head");
    write(&s.dir, "f.txt", b"one\n");
    commit_all(&s.dir, "first");
    let oid = rev_parse(&s.dir, "HEAD").unwrap().unwrap();
    assert_eq!(oid.as_str(), head(&s.dir));
    assert_eq!(Some(oid), Oid::parse(&head(&s.dir)));
}

#[test]
fn rev_parse_of_an_absent_oid_or_name_is_none() {
    let s = repo("rev_parse_absent");
    write(&s.dir, "f.txt", b"one\n");
    commit_all(&s.dir, "first");
    assert_eq!(rev_parse(&s.dir, "0123456789abcdef0123456789abcdef01234567").unwrap(), None);
    assert_eq!(rev_parse(&s.dir, "no-such-branch").unwrap(), None);
}

#[test]
fn rev_parse_of_an_option_shaped_rev_is_none() {
    // Taken as an option, `--help` would print usage; after `--end-of-options` it is a rev that does not exist.
    let s = repo("rev_parse_option_shaped");
    write(&s.dir, "f.txt", b"one\n");
    commit_all(&s.dir, "first");
    assert_eq!(rev_parse(&s.dir, "--help").unwrap(), None);
}

/// Base: f.txt "one".."five"; g.txt "a b c"; "dir/c d.txt" "x"; gone.txt.
fn added_lines_repo(test: &str) -> (Scratch, String) {
    let s = repo(test);
    write(&s.dir, "f.txt", b"one\ntwo\nthree\nfour\nfive\n");
    write(&s.dir, "g.txt", b"a\nb\nc\n");
    write(&s.dir, "dir/c d.txt", b"x\n");
    write(&s.dir, "gone.txt", b"gone\n");
    commit_all(&s.dir, "base");
    let base = head(&s.dir);
    // New f.txt: 1 one, 2 TWO, 3 three, 4 four, 5 four-a, 6 four-b (line 2 replaced, two lines after 4, 5 deleted).
    write(&s.dir, "f.txt", b"one\nTWO\nthree\nfour\nfour-a\nfour-b\n");
    // New g.txt: a, c (only a removal).
    write(&s.dir, "g.txt", b"a\nc\n");
    // New "dir/c d.txt": 1 x, 2 y.
    write(&s.dir, "dir/c d.txt", b"x\ny\n");
    fs::remove_file(s.dir.join("gone.txt")).unwrap();
    (s, base)
}

#[test]
fn added_lines_gives_new_file_ranges_per_file() {
    let (s, base) = added_lines_repo("added_lines_whole_repo");
    let got = added_lines(&s.dir, &base, &[]).unwrap();
    let want: BTreeMap<PathBuf, Vec<Range<u32>>> = BTreeMap::from([
        (p("dir/c d.txt"), vec![2..3]),
        (p("f.txt"), vec![2..3, 5..7]),
        (p("g.txt"), vec![]),
    ]);
    assert_eq!(got, want);
}

#[test]
fn added_lines_leaves_out_a_deleted_file() {
    let (s, base) = added_lines_repo("added_lines_deleted");
    let got = added_lines(&s.dir, &base, &[]).unwrap();
    assert!(!got.contains_key(&p("gone.txt")), "deleted file present: {got:?}");
}

#[test]
fn added_lines_with_only_removals_is_present_and_empty() {
    let (s, base) = added_lines_repo("added_lines_only_removals");
    let got = added_lines(&s.dir, &base, &[]).unwrap();
    assert_eq!(got.get(&p("g.txt")), Some(&vec![]));
}

#[test]
fn added_lines_of_a_path_with_a_space() {
    let (s, base) = added_lines_repo("added_lines_space");
    let got = added_lines(&s.dir, &base, &[]).unwrap();
    assert_eq!(got.get(&p("dir/c d.txt")), Some(&vec![2..3]));
}

#[test]
fn added_lines_limited_to_the_paths_given() {
    let (s, base) = added_lines_repo("added_lines_paths");
    let got = added_lines(&s.dir, &base, &[p("f.txt")]).unwrap();
    assert_eq!(got, BTreeMap::from([(p("f.txt"), vec![2..3, 5..7])]));
}

/// Committed f.txt holds a non-UTF-8 byte and no trailing newline; sub/g.txt sits in a subdirectory.
fn blob_repo(test: &str) -> Scratch {
    let s = repo(test);
    write(&s.dir, "f.txt", b"bin\xffary\nlast line");
    write(&s.dir, "sub/g.txt", b"in sub\n");
    commit_all(&s.dir, "base");
    s
}

#[test]
fn blob_at_head_is_the_committed_bytes() {
    let s = blob_repo("blob_at_head");
    write(&s.dir, "f.txt", b"working tree differs\n");
    assert_eq!(blob_at(&s.dir, "HEAD", Path::new("f.txt")).unwrap(), Some(b"bin\xffary\nlast line".to_vec()));
}

#[test]
fn blob_at_of_a_subdir_file_by_repo_relative_path() {
    let s = blob_repo("blob_at_subdir");
    assert_eq!(blob_at(&s.dir, "HEAD", Path::new("sub/g.txt")).unwrap(), Some(b"in sub\n".to_vec()));
}

#[test]
fn blob_at_of_a_missing_path_is_none() {
    let s = blob_repo("blob_at_missing_path");
    assert_eq!(blob_at(&s.dir, "HEAD", Path::new("absent.txt")).unwrap(), None);
}

#[test]
fn blob_at_a_missing_rev_is_exit_128() {
    let s = blob_repo("blob_at_missing_rev");
    match blob_at(&s.dir, "no-such-branch", Path::new("f.txt")) {
        Err(GitError::Exit { code, .. }) => assert_eq!(code, Some(128)),
        other => panic!("expected Exit, got {other:?}"),
    }
}

#[test]
fn blob_at_of_a_dot_slash_path_is_relative_to_repo() {
    let s = repo("blob_at_dot_slash");
    // A top-relative read would return these different bytes.
    write(&s.dir, "name", b"top\n");
    write(&s.dir, "sub/name", b"in sub\n");
    commit_all(&s.dir, "base");
    let rev = head(&s.dir);
    assert_eq!(blob_at(&s.dir.join("sub"), &rev, Path::new("./name")).unwrap(), Some(b"in sub\n".to_vec()));
}

/// Committed a.sh, sub/b.sh, c.txt and gone.sh; gone.sh is deleted, d.sh untracked and e.sh ignored.
fn tracked_repo(test: &str) -> Scratch {
    let s = repo(test);
    write(&s.dir, "a.sh", b"a\n");
    write(&s.dir, "sub/b.sh", b"b\n");
    write(&s.dir, "c.txt", b"c\n");
    write(&s.dir, "gone.sh", b"gone\n");
    commit_all(&s.dir, "base");
    fs::remove_file(s.dir.join("gone.sh")).unwrap();
    write(&s.dir, "d.sh", b"untracked\n");
    write(&s.dir, ".git/info/exclude", b"e.sh\n");
    write(&s.dir, "e.sh", b"ignored\n");
    s
}

#[test]
fn tracked_files_glob_matches_tracked_shell_files_across_directories() {
    let s = tracked_repo("tracked_glob");
    assert_eq!(tracked_files(&s.dir, &[OsString::from("*.sh")]).unwrap(), vec![p("a.sh"), p("sub/b.sh")]);
}

#[test]
fn tracked_files_without_pathspecs_lists_existing_tracked_files() {
    let s = tracked_repo("tracked_all");
    assert_eq!(tracked_files::<&str>(&s.dir, &[]).unwrap(), vec![p("a.sh"), p("c.txt"), p("sub/b.sh")]);
}

#[test]
fn tracked_files_from_subdir_without_pathspecs_is_top_relative() {
    let s = tracked_repo("tracked_subdir_all");
    assert_eq!(tracked_files::<&str>(&s.dir.join("sub"), &[]).unwrap(), vec![p("sub/b.sh")]);
}

#[test]
fn tracked_files_from_subdir_resolves_pathspecs_relative_to_subdir() {
    let s = tracked_repo("tracked_subdir_glob");
    assert_eq!(tracked_files(&s.dir.join("sub"), &["*.sh"]).unwrap(), vec![p("sub/b.sh")]);
}

#[test]
fn tracked_files_with_unmatched_pathspec_is_empty() {
    let s = tracked_repo("tracked_no_match");
    assert_eq!(tracked_files(&s.dir, &["*.nothing"]).unwrap(), Vec::<PathBuf>::new());
}

#[test]
fn tracked_files_of_a_plain_dir_is_exit_128() {
    let s = scratch("tracked_plain_dir");
    match tracked_files::<&str>(&s.dir, &[]) {
        Err(GitError::Exit { code, .. }) => assert_eq!(code, Some(128)),
        other => panic!("expected Exit, got {other:?}"),
    }
}

#[test]
fn tracked_files_lists_an_unmerged_file_once() {
    let s = repo("tracked_unmerged");
    write(&s.dir, "u.txt", b"working tree\n");
    write(&s.dir, ".git/base", b"base\n");
    write(&s.dir, ".git/ours", b"ours\n");
    write(&s.dir, ".git/theirs", b"theirs\n");
    let base = String::from_utf8(git(&s.dir, &["hash-object", "-w", ".git/base"])).unwrap();
    let ours = String::from_utf8(git(&s.dir, &["hash-object", "-w", ".git/ours"])).unwrap();
    let theirs = String::from_utf8(git(&s.dir, &["hash-object", "-w", ".git/theirs"])).unwrap();
    let index = format!(
        "100644 {} 1\tu.txt\n100644 {} 2\tu.txt\n100644 {} 3\tu.txt\n",
        base.trim(), ours.trim(), theirs.trim()
    );
    write(&s.dir, ".git/index-info", index.as_bytes());
    let out = Command::new("git")
        .arg("-C")
        .arg(&s.dir)
        .args(["update-index", "--index-info"])
        .env_remove("GIT_DIR")
        .env_remove("GIT_WORK_TREE")
        .env_remove("GIT_INDEX_FILE")
        .env_remove("GIT_COMMON_DIR")
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .stdin(fs::File::open(s.dir.join(".git/index-info")).unwrap())
        .output()
        .unwrap();
    assert!(out.status.success(), "setup git update-index failed: {}", String::from_utf8_lossy(&out.stderr));
    assert_eq!(tracked_files::<&str>(&s.dir, &[]).unwrap(), vec![p("u.txt")]);
}

#[test]
fn tracked_files_excludes_skip_worktree_entries() {
    let s = tracked_repo("tracked_skip_worktree");
    git(&s.dir, &["update-index", "--skip-worktree", "c.txt"]);
    fs::remove_file(s.dir.join("c.txt")).unwrap();
    assert_eq!(tracked_files::<&str>(&s.dir, &[]).unwrap(), vec![p("a.sh"), p("sub/b.sh")]);
}

#[test]
fn tracked_files_drops_a_skip_worktree_entry_still_on_disk() {
    let s = tracked_repo("tracked_skip_worktree_on_disk");
    git(&s.dir, &["update-index", "--skip-worktree", "c.txt"]);
    assert_eq!(tracked_files::<&str>(&s.dir, &[]).unwrap(), vec![p("a.sh"), p("sub/b.sh")]);
}

/// Committed: tracked.txt, sub/tracked2.txt. Then, uncommitted: .gitignore naming ignored.txt, ignored.txt, new.txt,
/// sub/u.txt.
fn untracked_repo(test: &str) -> Scratch {
    let s = repo(test);
    write(&s.dir, "tracked.txt", b"t\n");
    write(&s.dir, "sub/tracked2.txt", b"t2\n");
    commit_all(&s.dir, "base");
    write(&s.dir, ".gitignore", b"ignored.txt\n");
    write(&s.dir, "ignored.txt", b"i\n");
    write(&s.dir, "new.txt", b"n\n");
    write(&s.dir, "sub/u.txt", b"u\n");
    s
}

#[test]
fn untracked_files_lists_untracked_not_ignored_relative_to_the_top() {
    let s = untracked_repo("untracked_files_top");
    // The uncommitted .gitignore is itself untracked and not ignored, so it is listed.
    assert_eq!(untracked_files(&s.dir).unwrap(), vec![p(".gitignore"), p("new.txt"), p("sub/u.txt")]);
}

#[test]
fn untracked_files_from_a_subdir_is_the_whole_tree_relative_to_the_top() {
    let s = untracked_repo("untracked_files_subdir");
    assert_eq!(untracked_files(&s.dir.join("sub")).unwrap(), vec![p(".gitignore"), p("new.txt"), p("sub/u.txt")]);
}

#[test]
fn untracked_files_of_a_clean_tree_is_empty() {
    let s = repo("untracked_files_clean");
    write(&s.dir, "a.txt", b"a\n");
    commit_all(&s.dir, "base");
    assert_eq!(untracked_files(&s.dir).unwrap(), Vec::<PathBuf>::new());
}

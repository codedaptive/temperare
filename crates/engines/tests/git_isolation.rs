//! The shared git helper, run with `GIT_DIR` and `GIT_INDEX_FILE` pointing at another repository as a git hook
//! leaves them, works in its own throwaway repository and leaves the other repository's `.git` tree unchanged.

use std::fs;
use std::path::{Path, PathBuf};

#[path = "support/git.rs"]
mod git_support;

struct TempFolder(PathBuf);

impl TempFolder {
    fn new(name: &str) -> Self {
        let path = std::env::temp_dir().join(format!("engines-git-isolation-{}-{name}", std::process::id()));
        fs::create_dir(&path).unwrap();
        Self(fs::canonicalize(path).unwrap())
    }
}

impl Drop for TempFolder {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).unwrap();
    }
}

fn snapshot(root: &Path) -> Vec<(PathBuf, Vec<u8>)> {
    fn collect(root: &Path, dir: &Path, files: &mut Vec<(PathBuf, Vec<u8>)>) {
        for entry in fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                collect(root, &path, files);
            } else {
                files.push((path.strip_prefix(root).unwrap().to_path_buf(), fs::read(&path).unwrap()));
            }
        }
    }
    let mut files = Vec::new();
    collect(root, root, &mut files);
    files.sort();
    files
}

#[test]
fn the_git_helper_leaves_an_inherited_callers_repository_untouched() {
    let caller = TempFolder::new("caller");
    let out = procio::run(&procio::git_cmd(&caller.0, vec!["init".into(), "-q".into()])).unwrap();
    assert_eq!(out.code, Some(0), "{}", String::from_utf8_lossy(&out.stderr));
    let before = snapshot(&caller.0.join(".git"));

    std::env::set_var("GIT_DIR", caller.0.join(".git"));
    std::env::set_var("GIT_INDEX_FILE", caller.0.join(".git/index"));
    let second = TempFolder::new("second");
    let result = std::panic::catch_unwind(|| {
        git_support::git(&second.0, &["init", "-q"], b"");
        fs::write(second.0.join("x.sh"), b"echo hello\n").unwrap();
        git_support::git(&second.0, &["add", "x.sh"], b"");
    });
    std::env::remove_var("GIT_DIR");
    std::env::remove_var("GIT_INDEX_FILE");

    assert!(before == snapshot(&caller.0.join(".git")), "caller's .git tree changed");
    assert!(result.is_ok(), "git helper calls failed");
    let out = procio::run(&procio::git_cmd(&second.0, vec!["ls-files".into()])).unwrap();
    assert_eq!(out.code, Some(0), "{}", String::from_utf8_lossy(&out.stderr));
    assert_eq!(out.stdout, b"x.sh\n");
}

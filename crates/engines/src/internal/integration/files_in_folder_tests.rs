//! Folder listings exclude tracked paths reached through a symlinked folder.

use crate::files_in_folder::files_in_folder;
use rules::RelPath;
use std::fs;
use std::os::unix::fs::symlink;
use std::path::PathBuf;

#[path = "../../../tests/support/git.rs"]
mod git_support;

struct TempDir(PathBuf);

impl TempDir {
    fn new(name: &str) -> Self {
        let path = std::env::temp_dir().join(format!("engines-zit-{}-folder-files-{name}", std::process::id()));
        fs::create_dir(&path).unwrap();
        Self(fs::canonicalize(path).unwrap())
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[test]
fn files_in_folder_excludes_files_under_a_symlinked_folder() {
    let repo = TempDir::new("repo");
    let outside = TempDir::new("outside");
    let root = &repo.0;
    git_support::git(root, &["init", "-q"], b"");
    fs::create_dir(root.join("real")).unwrap();
    fs::create_dir(root.join("pkg")).unwrap();
    fs::write(root.join("real/keep.py"), "keep\n").unwrap();
    fs::write(root.join("pkg/a.py"), "tracked\n").unwrap();
    git_support::git(root, &["add", "-A"], b"");
    git_support::git(root, &["-c", "user.name=Test", "-c", "user.email=test@example.invalid", "commit", "-q", "-m", "base"], b"");
    fs::remove_dir_all(root.join("pkg")).unwrap();
    fs::write(outside.0.join("a.py"), "outside\n").unwrap();
    symlink(&outside.0, root.join("pkg")).unwrap();

    assert_eq!(files_in_folder(root), Some(vec![RelPath::new("real/keep.py")]));
}

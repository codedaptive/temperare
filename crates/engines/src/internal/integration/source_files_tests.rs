//! Source files exclude tracked paths reached through a symlinked folder.

use crate::source_files::source_files;
use std::fs;
use std::os::unix::fs::symlink;
use std::path::PathBuf;

#[path = "../../../tests/support/git.rs"]
mod git_support;

struct TempDir(PathBuf);

impl TempDir {
    fn new(name: &str) -> Self {
        let path = std::env::temp_dir().join(format!("engines-zit-{}-source-files-{name}", std::process::id()));
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
fn source_files_excludes_files_under_a_symlinked_folder() {
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

    assert_eq!(source_files(root, |path| path.extension().is_some_and(|ext| ext == "py")), Ok(vec![PathBuf::from("real/keep.py")]));
}

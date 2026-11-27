//! A throwaway git repository for one acceptance case: made under the temp folder, removed when dropped.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::env::clean_env;
use crate::run_clean::run_clean;

static NEXT: AtomicUsize = AtomicUsize::new(0);

pub struct Repo {
    root: PathBuf,
}

impl Repo {
    /// An empty repository on branch main under the temp folder, with a committer set and signing off.
    pub fn new() -> Repo {
        Repo::new_in(&std::env::temp_dir())
    }

    /// As `new`, made in `parent` (a case that needs the repository in a given place, such as /private/tmp).
    pub fn new_in(parent: &Path) -> Repo {
        let n = NEXT.fetch_add(1, Ordering::SeqCst);
        let root = parent.join(format!("cctao-accept-{}-{}", std::process::id(), n));
        fs::create_dir_all(&root).expect("make the throwaway repository");
        let repo = Repo { root };
        repo.git(&["init", "-q", "-b", "main"]);
        repo.git(&["config", "user.email", "accept@example.invalid"]);
        repo.git(&["config", "user.name", "accept"]);
        repo.git(&["config", "commit.gpgsign", "false"]);
        repo
    }

    pub fn path(&self) -> &Path {
        &self.root
    }

    /// Writes `text` to `rel`, making its folders.
    pub fn write(&self, rel: &str, text: &str) {
        let file = self.root.join(rel);
        if let Some(dir) = file.parent() {
            fs::create_dir_all(dir).expect("make the file's folder");
        }
        fs::write(&file, text).expect("write the file");
    }

    /// Commits everything and returns the new commit's id.
    pub fn commit(&self, message: &str) -> String {
        self.git(&["add", "-A"]);
        self.git(&["commit", "-q", "-m", message]);
        self.git(&["rev-parse", "HEAD"])
    }

    /// Runs git here in the clean environment; panics on failure; returns stdout trimmed.
    pub fn git(&self, args: &[&str]) -> String {
        let out = run_clean("git", args, &self.root, clean_env(None));
        assert!(
            out.code == Some(0),
            "git {args:?} failed: {}",
            String::from_utf8_lossy(&out.stderr)
        );
        String::from_utf8_lossy(&out.stdout).trim().to_string()
    }
}

impl Drop for Repo {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

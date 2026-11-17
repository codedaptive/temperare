//! procio's zone integration test that an inherited `GIT_DIR` and `GIT_WORK_TREE` do not leak into its git runs.
//! Its own test binary with one test, because it sets the process environment.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use procio::{changed_files, rev_parse, toplevel};

/// A scratch dir under the temp dir, unique to this process and name; removed when the guard drops.
struct Scratch {
    dir: PathBuf,
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.dir);
    }
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

/// A repository on `main` holding one commit of `file` with `content`.
fn repo(name: &str, file: &str, content: &[u8]) -> Scratch {
    let dir = std::env::temp_dir().join(format!("procio-zit-git-env-{}-{name}", std::process::id()));
    fs::create_dir(&dir).unwrap();
    let s = Scratch { dir };
    git(&s.dir, &["init", "-q", "-b", "main"]);
    git(&s.dir, &["config", "user.name", "procio zit"]);
    git(&s.dir, &["config", "user.email", "zit@procio.invalid"]);
    git(&s.dir, &["config", "commit.gpgsign", "false"]);
    fs::write(s.dir.join(file), content).unwrap();
    git(&s.dir, &["add", "-A"]);
    git(&s.dir, &["commit", "-q", "-m", name]);
    s
}

fn head(dir: &Path) -> String {
    String::from_utf8(git(dir, &["rev-parse", "HEAD"])).unwrap().trim().to_string()
}

#[test]
fn inherited_git_dir_and_work_tree_do_not_leak() {
    let a = repo("a", "a.txt", b"a\n");
    let b = repo("b", "b.txt", b"b\n");
    let a_head = head(&a.dir);
    assert_ne!(a_head, head(&b.dir));
    fs::write(a.dir.join("a.txt"), b"a changed\n").unwrap();
    fs::write(b.dir.join("b.txt"), b"b changed\n").unwrap();
    std::env::set_var("GIT_DIR", b.dir.join(".git"));
    std::env::set_var("GIT_WORK_TREE", &b.dir);

    assert_eq!(toplevel(&a.dir).unwrap(), fs::canonicalize(&a.dir).unwrap());
    assert_eq!(rev_parse(&a.dir, "HEAD").unwrap().unwrap().as_str(), a_head);
    assert_eq!(changed_files(&a.dir, &a_head).unwrap(), vec![PathBuf::from("a.txt")]);
}

//! ZITs: case repositories do not inherit another repository or a global commit hook.

use cli::BaselineWorld;
use std::ffi::OsString;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

const CHILD: &str = "TEMPERARE_BASELINE_GIT_ENV_CHILD";
static NEXT: AtomicU64 = AtomicU64::new(0);

struct TempFolder(PathBuf);

impl TempFolder {
    fn new() -> Self {
        loop {
            let n = NEXT.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir().join(format!("tm-baseline-git-env-{}-{n}", std::process::id()));
            match fs::create_dir(&path) {
                Ok(()) => return Self(path),
                Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => continue,
                Err(e) => panic!("cannot make {}: {e}", path.display()),
            }
        }
    }
}

impl Drop for TempFolder {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn clean_command(program: impl Into<OsString>, root: &Path) -> procio::Cmd {
    procio::Cmd {
        cwd: Some(root.to_path_buf()),
        env_remove: std::env::vars_os().map(|(name, _)| name).collect(),
        env: vec![
            ("PATH".into(), std::env::var_os("PATH").unwrap_or_else(|| "/usr/bin:/bin".into())),
            ("HOME".into(), root.as_os_str().into()),
            ("TMPDIR".into(), root.as_os_str().into()),
            ("GIT_CONFIG_NOSYSTEM".into(), "1".into()),
        ],
        ..procio::Cmd::new(program)
    }
}

fn git(root: &Path, args: &[&str]) -> String {
    let mut cmd = clean_command("git", root);
    cmd.args = args.iter().map(OsString::from).collect();
    let output = procio::run(&cmd).unwrap();
    assert_eq!(output.code, Some(0), "git {args:?}: {}", String::from_utf8_lossy(&output.stderr));
    String::from_utf8(output.stdout).unwrap()
}

fn child(root: &Path, name: &str, extra: Vec<(OsString, OsString)>) {
    let mut cmd = clean_command(std::env::current_exe().unwrap(), root);
    cmd.args = vec!["--exact".into(), format!("baseline_git_env::{name}").into(), "--nocapture".into()];
    cmd.env.push((CHILD.into(), "1".into()));
    cmd.env.extend(extra);
    let output = procio::run(&cmd).unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.code, Some(0), "child failed:\nstdout: {stdout}\nstderr: {stderr}");
    assert!(stdout.contains("1 passed"), "child ran no test:\nstdout: {stdout}\nstderr: {stderr}");
}

#[test]
fn case_git_steps_leave_an_inherited_git_dir_alone() {
    if std::env::var_os(CHILD).is_some() {
        let world = BaselineWorld::real();
        let folder = (world.make_folder)().expect("make case folder");
        let file = folder.join("a.py");
        (world.write_file)(&file, "x = 1\n").expect("write base file");
        let base = (world.commit_all)(&folder).expect("commit case base");
        (world.write_file)(&file, "x = 1\ny = 2\n").expect("write changed file");
        let added = (world.added_since)(&folder, &base).expect("read added lines");
        assert_eq!(added.get(Path::new("a.py")), Some(&vec![2..3]));
        (world.remove_folder)(&folder).expect("remove case folder");
        return;
    }

    let root = TempFolder::new();
    let repo = root.0.join("other-repo");
    fs::create_dir(&repo).unwrap();
    git(&repo, &["init", "-q"]);
    fs::write(repo.join("original.txt"), "original\n").unwrap();
    git(&repo, &["add", "-A"]);
    git(&repo, &["-c", "user.name=Test", "-c", "user.email=test@localhost", "-c", "commit.gpgsign=false", "commit", "-q", "-m", "original"]);
    let head = git(&repo, &["rev-parse", "HEAD"]);
    let index = fs::read(repo.join(".git/index")).unwrap();
    child(&root.0, "case_git_steps_leave_an_inherited_git_dir_alone", vec![
        ("GIT_DIR".into(), repo.join(".git").into_os_string()),
        ("GIT_INDEX_FILE".into(), repo.join(".git/index").into_os_string()),
    ]);
    assert_eq!(git(&repo, &["rev-parse", "HEAD"]), head, "other repository HEAD changed");
    assert_eq!(fs::read(repo.join(".git/index")).unwrap(), index, "other repository index changed");
}

#[test]
fn a_failing_global_pre_commit_hook_does_not_stop_the_case_commit() {
    if std::env::var_os(CHILD).is_some() {
        let world = BaselineWorld::real();
        let folder = (world.make_folder)().expect("make case folder");
        (world.write_file)(&folder.join("a.py"), "x = 1\n").expect("write case file");
        let result = (world.commit_all)(&folder);
        assert!(result.is_ok(), "commit case base: {result:?}");
        let id = result.unwrap();
        assert_eq!(id.len(), 40, "commit id: {id}");
        assert!(id.bytes().all(|b| b.is_ascii_hexdigit()), "commit id: {id}");
        (world.remove_folder)(&folder).expect("remove case folder");
        return;
    }

    let root = TempFolder::new();
    let hooks = root.0.join("hooks");
    fs::create_dir(&hooks).unwrap();
    let hook = hooks.join("pre-commit");
    fs::write(&hook, "#!/bin/sh\necho baseline-hook-marker >&2\nexit 1\n").unwrap();
    fs::set_permissions(&hook, fs::Permissions::from_mode(0o755)).unwrap();
    let config = root.0.join("gitconfig");
    fs::write(&config, format!("[core]\n\thooksPath = {}\n", hooks.display())).unwrap();
    child(&root.0, "a_failing_global_pre_commit_hook_does_not_stop_the_case_commit", vec![
        ("GIT_CONFIG_GLOBAL".into(), config.into_os_string()),
    ]);
}

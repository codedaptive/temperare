//! ZIT: tm runs for real with stub engines in a throwaway git folder under the system temp folder, removed on drop.

use procio::{Cmd, Output};
use serde_json::{json, Value};
use std::ffi::OsString;
use std::fs;
use std::os::unix::fs::{symlink, PermissionsExt};
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT: AtomicU64 = AtomicU64::new(0);

struct Repo(PathBuf);

impl Repo {
    fn new() -> Self {
        let dir = loop {
            let n = NEXT.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir().join(format!("tm-local-eslint-{}-{n}", std::process::id()));
            match fs::create_dir(&path) {
                Ok(()) => break fs::canonicalize(path).unwrap(),
                Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => continue,
                Err(e) => panic!("cannot make {}: {e}", path.display()),
            }
        };
        let repo = Self(dir);
        fs::create_dir_all(repo.0.join("node_modules/.bin")).unwrap();
        fs::create_dir(repo.0.join("stubs")).unwrap();
        fs::create_dir(repo.0.join("home")).unwrap();
        fs::write(repo.0.join("temperare.toml"), r#"version = 1
[[rule]]
id = "js-no-var"
engine = "eslint"
check = "no-var"
paths = ["**/*.js"]
fix = "Declare with let or const."
"#).unwrap();
        fs::write(repo.0.join("a.js"), "const a = 1;\n").unwrap();
        let eslint = repo.0.join("node_modules/.bin/eslint");
        fs::write(&eslint, "#!/bin/sh\nif [ \"$1\" = --version ]; then printf 'v10.15.0\\n'; else printf '[]\\n'; fi\n").unwrap();
        fs::set_permissions(eslint, fs::Permissions::from_mode(0o755)).unwrap();
        let node = repo.0.join("stubs/node");
        fs::write(&node, "#!/bin/sh\nexit 0\n").unwrap();
        fs::set_permissions(node, fs::Permissions::from_mode(0o755)).unwrap();
        let output = procio::run(&Cmd { args: vec!["--exec-path".into()], ..Cmd::new("git") }).unwrap();
        assert_eq!(output.code, Some(0), "git --exec-path: {}", String::from_utf8_lossy(&output.stderr));
        let git = PathBuf::from(String::from_utf8(output.stdout).unwrap().trim()).join("git");
        symlink(git, repo.0.join("stubs/git")).unwrap();
        for args in [&["init", "-q"][..], &["add", "a.js"][..]] {
            let output = repo.run("git", args);
            assert_eq!(output.code, Some(0), "git {args:?}: {}", String::from_utf8_lossy(&output.stderr));
        }
        repo
    }

    fn run(&self, program: &str, args: &[&str]) -> Output {
        procio::run(&Cmd {
            args: args.iter().map(OsString::from).collect(),
            cwd: Some(self.0.clone()),
            env_remove: std::env::vars_os().map(|(name, _)| name).collect(),
            env: vec![
                ("PATH".into(), std::env::join_paths([self.0.join("stubs"), PathBuf::from("/usr/bin"), PathBuf::from("/bin")]).unwrap()),
                ("HOME".into(), self.0.join("home").into_os_string()),
                ("GIT_CONFIG_NOSYSTEM".into(), "1".into()),
                ("GIT_CONFIG_GLOBAL".into(), "/dev/null".into()),
                ("GIT_CEILING_DIRECTORIES".into(), self.0.parent().unwrap().into()),
            ],
            ..Cmd::new(program)
        }).unwrap()
    }
}

impl Drop for Repo {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn rules_lists_local_eslint(json_output: bool) {
    let repo = Repo::new();
    let args: &[&str] = if json_output { &["rules", "--json"] } else { &["rules"] };
    let output = repo.run(env!("CARGO_BIN_EXE_temperare"), args);
    assert_eq!(output.code, Some(0), "{}", String::from_utf8_lossy(&output.stderr));
    if json_output {
        let listing: Value = serde_json::from_slice(&output.stdout).unwrap();
        let rule = listing["rules"].as_array().unwrap().iter().find(|rule| rule["id"] == "js-no-var").unwrap();
        assert_eq!(rule.get("available"), Some(&json!(true)), "{listing}");
        assert_eq!(rule.get("why"), Some(&Value::Null), "{listing}");
    } else {
        let listing = String::from_utf8(output.stdout).unwrap();
        assert!(listing.lines().any(|line| line.contains("js-no-var") && line.ends_with("  available")), "{listing}");
    }
}

macro_rules! rules_case {
    ($name:ident, $json:expr) => {
        #[test]
        fn $name() { rules_lists_local_eslint($json); }
    };
}

rules_case!(rules_text_lists_local_eslint_as_available, false);
rules_case!(rules_json_lists_local_eslint_as_available, true);

#[test]
fn check_reports_local_eslint_and_version() {
    let repo = Repo::new();
    let output = repo.run(env!("CARGO_BIN_EXE_temperare"), &["check", "--json"]);
    let report: Value = serde_json::from_slice(&output.stdout).unwrap_or_else(|e| {
        panic!("{e}; stdout: {}; stderr: {}", String::from_utf8_lossy(&output.stdout), String::from_utf8_lossy(&output.stderr))
    });
    assert_eq!(report["engines"], json!([{ "engine": "eslint", "version": "10.15.0" }]), "{report}");
}

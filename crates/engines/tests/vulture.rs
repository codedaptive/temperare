#![cfg(unix)]

use std::ffi::OsString;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

use engines::run_rules_versioned_at;
use rules::{Engine, RelPath, RuleFile, RuleOutcome};

#[path = "support/git.rs"]
mod git_support;

struct TempFolder(PathBuf);

impl TempFolder {
    fn new(name: &str) -> Self {
        let path = std::env::temp_dir().join(format!("engines-zit-{}-vulture-{name}", std::process::id()));
        fs::create_dir(&path).unwrap();
        Self(fs::canonicalize(path).unwrap())
    }
}

impl Drop for TempFolder {
    fn drop(&mut self) { fs::remove_dir_all(&self.0).unwrap(); }
}

struct PathGuard(Option<OsString>);

impl Drop for PathGuard {
    fn drop(&mut self) {
        match &self.0 {
            Some(path) => std::env::set_var("PATH", path),
            None => std::env::remove_var("PATH"),
        }
    }
}

#[test]
fn vulture_rule_reports_findings_clean_config_warnings_and_absence() {
    let original = PathGuard(std::env::var_os("PATH"));
    let path = original.0.as_ref().unwrap();
    std::env::set_var("PATH", std::env::join_paths(
        std::iter::once(Path::new("/opt/homebrew/bin").to_path_buf()).chain(std::env::split_paths(path))
    ).unwrap());
    let violation = "def dead():\n    pass\n";
    let cases = [
        ("violation", Some(violation), false, false, false, 1),
        ("clean", Some("def dead():\n    pass\ndead()\n"), false, false, false, 0),
        ("config", Some(violation), true, false, false, 1),
        ("warning", Some(violation), false, true, false, 1),
        ("no-python", None, false, false, false, 0),
        ("absent", Some(violation), false, false, true, 0),
        ("ignored-venv", Some("def clean():\n    pass\nclean()\n"), false, false, false, 0),
        ("tracked-broken", None, false, false, false, 0),
    ];
    let mut failed = Vec::new();
    for (name, source, config, warning, absent, count) in cases {
        let result = std::panic::catch_unwind(|| {
            let folder = TempFolder::new(name);
            if name == "ignored-venv" || name == "tracked-broken" {
                git_support::git(&folder.0, &["init", "-q"], b"");
                if name == "ignored-venv" {
                    fs::write(folder.0.join(".gitignore"), ".venv/\n").unwrap();
                    fs::create_dir_all(folder.0.join(".venv/lib")).unwrap();
                    fs::write(folder.0.join(".venv/lib/broken.py"), "def broken(:\n").unwrap();
                } else {
                    fs::write(folder.0.join("broken.py"), "def broken(:\n").unwrap();
                    git_support::git(&folder.0, &["add", "broken.py"], b"");
                }
            }
            if let Some(source) = source { fs::write(folder.0.join("example.py"), source).unwrap(); }
            if source.is_none() { fs::write(folder.0.join("notes.txt"), "no Python here\n").unwrap(); }
            if config { fs::write(folder.0.join("pyproject.toml"), "[tool.vulture]\nignore_names = [\"dead\"]\n").unwrap(); }
            if warning { fs::write(folder.0.join("warning.py"), "PAT = \"\\d+\"\nprint(PAT)\n").unwrap(); }
            fs::write(folder.0.join("temperare.toml"), "version = 1\n[[rule]]\nid = \"dead-function\"\nengine = \"vulture\"\ncheck = \"V103\"\nfix = \"Remove the function or call it.\"\n").unwrap();
            let file = RuleFile::read(&folder.0.join("temperare.toml")).unwrap();
            let _path_guard = if absent {
                let guard = PathGuard(std::env::var_os("PATH"));
                let without_vulture = std::env::join_paths(std::env::split_paths(guard.0.as_ref().unwrap()).filter(|dir| {
                    !dir.join("vulture").metadata().is_ok_and(|m| m.is_file() && m.permissions().mode() & 0o111 != 0)
                })).unwrap();
                std::env::set_var("PATH", without_vulture);
                Some(guard)
            } else { None };
            let runs = run_rules_versioned_at(&file, &folder.0, &[], None).runs;
            assert_eq!(runs.len(), 1, "{name}: {runs:?}");
            if name == "tracked-broken" {
                let RuleOutcome::Failed { stderr, .. } = &runs[0].outcome else {
                    panic!("{name}: expected failure, got {:?}", runs[0].outcome);
                };
                assert!(stderr.contains("broken.py"), "{name}: {stderr}");
                return;
            }
            if absent {
                assert_eq!(runs[0].outcome, RuleOutcome::NotAvailable {
                    rule: "dead-function".into(), engine: Engine::Vulture, why: "vulture not found on PATH".into(),
                }, "{name}");
                return;
            }
            let RuleOutcome::Findings(findings) = &runs[0].outcome else {
                panic!("{name}: expected findings, got {:?}", runs[0].outcome);
            };
            assert_eq!(findings.len(), count, "{name}: {findings:?}");
            if count == 1 {
                let finding = &findings[0];
                assert_eq!(finding.rule, "dead-function", "{name}");
                assert_eq!(finding.path, RelPath::new("example.py"), "{name}");
                assert_eq!(finding.line(), Some(1), "{name}");
                assert_eq!(finding.col, None, "{name}");
                assert_eq!(finding.what, "unused function 'dead'", "{name}");
                assert_eq!(finding.fix, "Remove the function or call it.", "{name}");
            }
        });
        if result.is_err() { failed.push(name); }
    }
    assert!(failed.is_empty(), "failed rows: {failed:?}");
}

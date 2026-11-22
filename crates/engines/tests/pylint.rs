#![cfg(unix)]

use std::ffi::OsString;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

use engines::run_rules_versioned_at;
use rules::{Engine, RelPath, RuleFile, RuleOutcome};

struct TempFolder(PathBuf);

impl TempFolder {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!("engines-zit-{}-pylint", std::process::id()));
        fs::create_dir(&path).unwrap();
        Self(fs::canonicalize(path).unwrap())
    }
}

impl Drop for TempFolder {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).unwrap();
    }
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
fn pylint_rule_reports_findings_clean_config_and_absence() {
    let folder = TempFolder::new();
    let original_path = PathGuard(std::env::var_os("PATH"));
    let path = original_path.0.as_ref().unwrap();
    std::env::set_var("PATH", std::env::join_paths(
        std::iter::once(Path::new("/opt/homebrew/bin").to_path_buf()).chain(std::env::split_paths(path))
    ).unwrap());
    fs::write(folder.0.join("pylint.rc"), "[MESSAGES CONTROL]\ndisable=C0121\n").unwrap();

    let violation = "import os\nvalue = None\nif value == None:\n    pass\n";
    let clean = "\"\"\"Doc.\"\"\"\nVALUE = None\nif VALUE is None:\n    pass\n";
    let cases = [
        ("violation", violation, false, false, 1),
        ("clean", clean, false, false, 0),
        ("config disables C0121", violation, true, false, 1),
        ("pylint absent", violation, false, true, 0),
    ];
    let mut failed = Vec::new();
    for (name, source, config, absent, count) in cases {
        let result = std::panic::catch_unwind(|| {
            fs::write(folder.0.join("example.py"), source).unwrap();
            fs::write(folder.0.join("temperare.toml"), format!(
                "version = 1\n[[rule]]\nid = \"none-comparison\"\nengine = \"pylint\"\ncheck = \"C0121\"\nfix = \"Use identity for None.\"\n{}",
                if config { "config = \"pylint.rc\"\n" } else { "" }
            )).unwrap();
            let file = RuleFile::read(&folder.0.join("temperare.toml")).unwrap();
            let _path_guard = if absent {
                let guard = PathGuard(std::env::var_os("PATH"));
                let without_pylint = std::env::join_paths(std::env::split_paths(guard.0.as_ref().unwrap()).filter(|dir| {
                    !dir.join("pylint").metadata().is_ok_and(|m| m.is_file() && m.permissions().mode() & 0o111 != 0)
                })).unwrap();
                std::env::set_var("PATH", without_pylint);
                Some(guard)
            } else {
                None
            };
            let runs = run_rules_versioned_at(&file, &folder.0, &[], None).runs;
            assert_eq!(runs.len(), 1, "{name}: {runs:?}");
            if absent {
                assert_eq!(runs[0].outcome, RuleOutcome::NotAvailable {
                    rule: "none-comparison".into(), engine: Engine::Pylint,
                    why: "pylint not found on PATH".into(),
                }, "{name}");
                return;
            }
            let RuleOutcome::Findings(findings) = &runs[0].outcome else {
                panic!("{name}: expected findings, got {:?}", runs[0].outcome);
            };
            assert_eq!(findings.len(), count, "{name}: {findings:?}");
            if count == 1 {
                let finding = &findings[0];
                assert_eq!(finding.rule, "none-comparison", "{name}");
                assert_eq!(finding.path, RelPath::new("example.py"), "{name}");
                assert_eq!(finding.line(), Some(3), "{name}");
                assert_eq!(finding.col, Some(4), "{name}");
                assert_eq!(finding.what, "Comparison 'value == None' should be 'value is None'", "{name}");
                assert_eq!(finding.fix, "Use identity for None.", "{name}");
            }
        });
        if result.is_err() {
            failed.push(name);
        }
    }
    assert!(failed.is_empty(), "failed rows: {failed:?}");
}

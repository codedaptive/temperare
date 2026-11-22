#![cfg(unix)]

use std::ffi::OsString;
use std::fs;
use std::os::unix::fs::{symlink, PermissionsExt};
use std::path::PathBuf;

use engines::{availability, run_rules_versioned_at, Availability};
use rules::{Engine, RelPath, RuleFile, RuleOutcome};

struct TempFolder(PathBuf);

impl TempFolder {
    fn new(case: &str) -> Self {
        let path = std::env::temp_dir().join(format!("engines-zit-{}-golangci-{case}", std::process::id()));
        fs::create_dir(&path).unwrap();
        Self(fs::canonicalize(path).unwrap())
    }
}

impl Drop for TempFolder {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).unwrap();
    }
}

struct EnvGuard(&'static str, Option<OsString>);

impl EnvGuard {
    fn set(name: &'static str, value: impl AsRef<std::ffi::OsStr>) -> Self {
        let previous = std::env::var_os(name);
        std::env::set_var(name, value);
        Self(name, previous)
    }
}

impl Drop for EnvGuard {
    fn drop(&mut self) {
        match &self.1 {
            Some(value) => std::env::set_var(self.0, value),
            None => std::env::remove_var(self.0),
        }
    }
}

struct SymlinkGuard(PathBuf);

impl Drop for SymlinkGuard {
    fn drop(&mut self) {
        fs::remove_file(&self.0).unwrap();
    }
}

#[test]
fn golangci_lint_reports_findings_and_unavailable_or_failed_runs() {
    assert_eq!(availability(Engine::GolangciLint), Availability::Available);
    let _environment = [
        EnvGuard::set("GOTOOLCHAIN", "auto"),
        EnvGuard::set("GOPROXY", "http://127.0.0.1:9"),
        EnvGuard::set("GOFLAGS", "-mod=mod"),
    ];
    let deprecated = "package sample\nimport \"strings\"\nfunc Title(s string) string {\n    return strings.Title(s)\n}\n";
    let clean = "package sample\n// Answer returns a constant.\nfunc Answer() int { return 42 }\n";
    let missing = "package sample\nimport \"example.invalid/none\"\nfunc Value() int { return none.X }\n";

    for (case, source, check, module) in [
        ("finding", deprecated, "staticcheck", "module example.com/finding\n\ngo 1.27\n"),
        ("clean", clean, "staticcheck", "module example.com/clean\n\ngo 1.27\n"),
        ("goflags", clean, "staticcheck", "module example.com/goflags\n"),
        ("proxy", missing, "staticcheck", "module example.com/proxy\n\ngo 1.27\n\nrequire example.invalid/none v1.0.0\n"),
        ("toolchain", deprecated, "staticcheck", "module example.com/toolchain\n\ngo 1.27\n\ntoolchain go1.27.99\n"),
        ("unknown", clean, "nosuchlinter", "module example.com/unknown\n\ngo 1.27\n"),
        ("absent", deprecated, "staticcheck", "module example.com/absent\n\ngo 1.27\n"),
    ] {
        let folder = TempFolder::new(case);
        fs::write(folder.0.join("go.mod"), module).unwrap();
        if case == "proxy" {
            fs::write(folder.0.join("go.sum"), "example.invalid/none v1.0.0 h1:AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=\nexample.invalid/none v1.0.0/go.mod h1:AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=\n").unwrap();
        }
        fs::write(folder.0.join("a.go"), source).unwrap();
        fs::write(folder.0.join("temperare.toml"), format!(r#"version = 1
[[rule]]
id = "go-static"
engine = "golangci-lint"
check = "{check}"
kind = "quality"
severity = "warning"
fix = "Use a replacement for the deprecated function."
"#)).unwrap();
        let file = RuleFile::read(&folder.0.join("temperare.toml")).unwrap();
        let link = (case == "finding").then(|| {
            let path = std::env::temp_dir().join(format!("engines-zit-{}-golangci-finding-link", std::process::id()));
            symlink(&folder.0, &path).unwrap();
            SymlinkGuard(path)
        });
        let _pwd = link.as_ref().map(|link| EnvGuard::set("PWD", &link.0));
        let _path = (case == "absent").then(|| {
            let previous = std::env::var_os("PATH").unwrap();
            let filtered = std::env::join_paths(std::env::split_paths(&previous).filter(|dir| {
                !dir.join("golangci-lint").metadata().is_ok_and(|m| m.is_file() && m.permissions().mode() & 0o111 != 0)
            })).unwrap();
            EnvGuard::set("PATH", filtered)
        });
        let runs = run_rules_versioned_at(&file, &folder.0, &[], None).runs;
        assert_eq!(runs.len(), 1, "{case}: {runs:?}");
        match case {
            "finding" => {
                let RuleOutcome::Findings(findings) = &runs[0].outcome else {
                    panic!("{case}: expected Findings, got {:?}", runs[0].outcome);
                };
                assert_eq!(findings.len(), 1, "{case}: {findings:?}");
                assert_eq!(findings[0].rule, "go-static");
                assert_eq!(findings[0].path, RelPath::new("a.go"));
                assert_eq!(findings[0].line(), Some(4));
                assert_eq!(findings[0].col, Some(12));
                assert!(findings[0].what.starts_with("SA1019:"), "{:?}", findings[0]);
                assert_eq!(findings[0].fix, "Use a replacement for the deprecated function.");
                assert!(runs[0].raw_chars.is_some());
            }
            "clean" => assert_eq!(runs[0].outcome, RuleOutcome::Findings(vec![]), "{case}"),
            "goflags" => {
                assert_eq!(runs[0].outcome, RuleOutcome::Findings(vec![]), "{case}");
                assert_eq!(fs::read(folder.0.join("go.mod")).unwrap(), b"module example.com/goflags\n");
            }
            "proxy" => {
                let RuleOutcome::Failed { stderr, .. } = &runs[0].outcome else {
                    panic!("{case}: expected Failed, got {:?}", runs[0].outcome);
                };
                assert!(stderr.contains("module lookup disabled by GOPROXY=off"), "{stderr}");
            }
            "toolchain" => {
                let RuleOutcome::Findings(findings) = &runs[0].outcome else {
                    panic!("{case}: expected Findings, got {:?}", runs[0].outcome);
                };
                assert_eq!(findings.len(), 1, "{case}: {findings:?}");
                assert!(findings[0].what.starts_with("SA1019:"), "{:?}", findings[0]);
            }
            "unknown" => {
                let RuleOutcome::NotAvailable { why, .. } = &runs[0].outcome else {
                    panic!("{case}: expected NotAvailable, got {:?}", runs[0].outcome);
                };
                assert!(why.contains("nosuchlinter"), "{why}");
            }
            "absent" => {
                assert_eq!(runs[0].outcome, RuleOutcome::NotAvailable {
                    rule: "go-static".into(),
                    engine: Engine::GolangciLint,
                    why: "golangci-lint not found on PATH".into(),
                });
                assert_eq!(runs[0].raw_chars, None);
            }
            _ => unreachable!(),
        }
    }
}

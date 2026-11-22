#![cfg(unix)]

use std::ffi::OsString;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::sync::Mutex;

use engines::{run_rules_versioned_at, NotAvailableCause};
use rules::{Engine, RelPath, RuleFile, RuleOutcome};

#[path = "support/git.rs"]
mod git_support;

static LOCK: Mutex<()> = Mutex::new(());

struct TempFolder(PathBuf);
impl TempFolder {
    fn new(case: &str) -> Self {
        let path = std::env::temp_dir().join(format!("engines-zit-{}-periphery-{case}", std::process::id()));
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

#[derive(Clone, Copy)]
enum Case { Found, Clean, NoPackage, Remote, NoEngine }

fn check_case(name: &str, case: Case) {
    let _lock = LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let saved_path = PathGuard(std::env::var_os("PATH"));
    let repo = TempFolder::new(name);
    git_support::git(&repo.0, &["init", "-q"], b"");
    fs::create_dir_all(repo.0.join("Sources/Probe")).unwrap();
    let source = if matches!(case, Case::Found) {
        "func unusedHelper() {}\nprint(\"hello\")\n"
    } else {
        "print(\"hello\")\n"
    };
    fs::write(repo.0.join("Sources/Probe/main.swift"), source).unwrap();
    let manifest = if matches!(case, Case::Remote) {
        "// swift-tools-version: 6.0\nimport PackageDescription\nlet package = Package(name: \"Probe\", dependencies: [.package(url: \"https://example.invalid/dep.git\", from: \"1.0.0\")], targets: [.executableTarget(name: \"Probe\")])\n"
    } else {
        "// swift-tools-version: 6.0\nimport PackageDescription\nlet package = Package(name: \"Probe\", targets: [.executableTarget(name: \"Probe\")])\n"
    };
    if !matches!(case, Case::NoPackage) {
        fs::write(repo.0.join("Package.swift"), manifest).unwrap();
    }
    fs::write(repo.0.join("temperare.toml"), "version = 1\n[[rule]]\nid = \"unused-code\"\nengine = \"periphery\"\ncheck = \"unused\"\npaths = [\"**\"]\nkind = \"quality\"\nseverity = \"warning\"\nfix = \"Remove unused code.\"\n").unwrap();
    git_support::git(&repo.0, &["add", "."], b"");
    if matches!(case, Case::NoEngine) {
        let filtered = std::env::join_paths(std::env::split_paths(saved_path.0.as_ref().unwrap()).filter(|dir| {
            !dir.join("periphery").metadata().is_ok_and(|m| m.is_file() && m.permissions().mode() & 0o111 != 0)
        })).unwrap();
        std::env::set_var("PATH", filtered);
    }
    let file = RuleFile::read(&repo.0.join("temperare.toml")).unwrap();
    let runs = run_rules_versioned_at(&file, &repo.0, &[], None).runs;
    assert_eq!(runs.len(), 1, "{name}: {runs:?}");
    let run = &runs[0];
    assert_eq!(run.rule, "unused-code");
    assert_eq!(run.engine, Engine::Periphery);
    match case {
        Case::Found => {
            let RuleOutcome::Findings(findings) = &run.outcome else { panic!("{name}: {run:?}") };
            assert_eq!(findings.len(), 1, "{findings:?}");
            assert_eq!(findings[0].rule, "unused-code");
            assert_eq!(findings[0].path, RelPath::new("Sources/Probe/main.swift"));
            assert_eq!(findings[0].line(), Some(1));
            assert_eq!(findings[0].col, Some(6));
            assert_eq!(findings[0].fix, "Remove unused code.");
            let mut names = fs::read_dir(&repo.0).unwrap().map(|e| e.unwrap().file_name()).collect::<Vec<_>>();
            names.sort();
            assert_eq!(names, [".git", "Package.swift", "Sources", "temperare.toml"].map(OsString::from));
            let sources = fs::read_dir(repo.0.join("Sources")).unwrap().map(|e| e.unwrap().file_name()).collect::<Vec<_>>();
            assert_eq!(sources, [OsString::from("Probe")]);
            let target = fs::read_dir(repo.0.join("Sources/Probe")).unwrap().map(|e| e.unwrap().file_name()).collect::<Vec<_>>();
            assert_eq!(target, [OsString::from("main.swift")]);
            assert_eq!(fs::read_to_string(repo.0.join("Package.swift")).unwrap(), manifest);
            assert_eq!(fs::read_to_string(repo.0.join("Sources/Probe/main.swift")).unwrap(), source);
            let prefix = format!("tm-periphery-{}-", std::process::id());
            assert!(!fs::read_dir(std::env::temp_dir()).unwrap().any(|e| e.unwrap().file_name().to_string_lossy().starts_with(&prefix)));
        }
        Case::Clean => assert_eq!(run.outcome, RuleOutcome::Findings(vec![])),
        Case::NoPackage | Case::Remote | Case::NoEngine => {
            let RuleOutcome::NotAvailable { why, .. } = &run.outcome else { panic!("{name}: {run:?}") };
            match case {
                Case::NoPackage => {
                    assert_eq!(run.not_available_cause, Some(NotAvailableCause::Other));
                    assert_eq!(why, "no Package.swift at or above the rule file's folder: Periphery builds the project, and tm runs it only on a Swift package");
                }
                Case::Remote => {
                    assert_eq!(run.not_available_cause, Some(NotAvailableCause::Other));
                    assert_eq!(why, "the package depends on dep, which tm would have to fetch: tm never fetches packages, so Periphery cannot build it here");
                    assert!(!repo.0.join(".build").exists());
                }
                Case::NoEngine => assert_eq!(run.not_available_cause, Some(NotAvailableCause::EngineNotOnPath)),
                _ => unreachable!(),
            }
        }
    }
}

macro_rules! cases {
    ($( $name:ident => $case:ident ),+ $(,)?) => {
        $(#[test] fn $name() { check_case(stringify!($name), Case::$case); })+
    };
}

cases! {
    found_unused_function => Found,
    clean_package => Clean,
    no_package_is_not_available => NoPackage,
    remote_dependency_is_not_available => Remote,
    missing_periphery_is_not_available => NoEngine,
}

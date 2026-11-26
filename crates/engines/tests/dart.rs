#![cfg(unix)]

use std::ffi::OsString;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use engines::run_rules_versioned_at;
use rules::{Engine, RelPath, RuleFile, RuleOutcome};

#[path = "support/git.rs"]
mod git_support;

static PATH_LOCK: Mutex<()> = Mutex::new(());
const PUBSPEC: &str = "name: probe\nenvironment:\n  sdk: ^3.0.0\n";
const VIOLATION: &str = "void main() {\n  var unused = 1;\n}\n";
const QUOTES: &str = "void main() {\n  print(\"x\");\n}\n";
const FIX: &str = "Fix the Dart diagnostic.";

struct TempFolder(PathBuf);

impl TempFolder {
    fn new(name: &str) -> Self {
        let path = fs::canonicalize(std::env::temp_dir()).unwrap()
            .join(format!("engines-zit-{}-dart-{name}", std::process::id()));
        fs::create_dir(&path).unwrap();
        Self(path)
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

#[derive(Clone, Copy, Debug)]
enum Case {
    Violation, Clean, LintEnabled, LintDisabled, Unknown, NoPubspec,
    UnresolvedInclude, UnfetchedPackage, NoFilesAbsent, Absent,
    ErrorsIgnore, Exclude, OptionsAboveRepo, GitignoredNestedOptions, IncludeOutsideRepo,
    EscapedIgnore, MultilineQuoted, EscapedInclude, FlowErrors, SymlinkedFolderOutside,
}

fn check_case(case: Case) {
    let outer = TempFolder::new(&format!("{case:?}"));
    let repo = if matches!(case, Case::OptionsAboveRepo | Case::IncludeOutsideRepo | Case::SymlinkedFolderOutside) {
        let repo = outer.0.join("repo");
        fs::create_dir(&repo).unwrap();
        repo
    } else {
        outer.0.clone()
    };
    git_support::git(&repo, &["init", "-q"], b"");

    if !matches!(case, Case::NoPubspec) {
        let pubspec = if matches!(case, Case::UnfetchedPackage) {
            format!("{PUBSPEC}dependencies:\n  path: ^1.9.0\n")
        } else {
            PUBSPEC.to_string()
        };
        fs::write(repo.join("pubspec.yaml"), pubspec).unwrap();
    }
    if matches!(case, Case::NoFilesAbsent) {
        fs::write(repo.join("notes.txt"), "no Dart files\n").unwrap();
    } else {
        fs::create_dir(repo.join("lib")).unwrap();
        let source = match case {
            Case::Clean => "void main() {\n  var unused = 1;\n  print(unused);\n}\n",
            Case::LintEnabled | Case::LintDisabled => QUOTES,
            Case::UnfetchedPackage => "import 'package:path/path.dart' as path;\nvoid main() {\n  print(path.basename('a/b'));\n}\n",
            _ => VIOLATION,
        };
        fs::write(repo.join("lib/a.dart"), source).unwrap();
    }
    if matches!(case, Case::GitignoredNestedOptions) {
        fs::write(repo.join(".gitignore"), "gen/\n").unwrap();
        fs::create_dir(repo.join("gen")).unwrap();
        fs::write(repo.join("gen/analysis_options.yaml"), "analyzer:\n  exclude:\n    - \"**\"\n").unwrap();
        fs::write(repo.join("gen/g.dart"), VIOLATION).unwrap();
    }
    if matches!(case, Case::IncludeOutsideRepo) {
        fs::write(outer.0.join("shared.yaml"), "linter:\n  rules:\n    - prefer_single_quotes\n").unwrap();
    }
    if matches!(case, Case::SymlinkedFolderOutside) {
        let outside = outer.0.join("outside");
        fs::create_dir(&outside).unwrap();
        fs::write(outside.join("analysis_options.yaml"), "analyzer:\n  errors:\n    unused_local_variable: ignore\n").unwrap();
        fs::write(outside.join("e.dart"), VIOLATION).unwrap();
        std::os::unix::fs::symlink(&outside, repo.join("ext")).unwrap();
    }
    if matches!(case, Case::EscapedInclude) {
        fs::write(repo.join("ign.yaml"), "analyzer:\n  errors:\n    unused_local_variable: ignore\n").unwrap();
    }
    let options = match case {
        Case::EscapedIgnore => Some("analyzer:\n  errors:\n    unused_local_variable: \"\\x69gnore\"\n"),
        Case::MultilineQuoted => Some("analyzer:\n  errors: {x: \"a\n    # b\", unused_local_variable: ignore}\n"),
        Case::EscapedInclude => Some("\"\\x69nclude\": ign.yaml\n"),
        Case::FlowErrors => Some("analyzer:\n  errors: {unused_local_variable: ignore}\n"),
        Case::LintEnabled => Some("linter:\n  rules:\n    - prefer_single_quotes\n"),
        Case::UnresolvedInclude => Some("include: package:lints/recommended.yaml\n"),
        Case::ErrorsIgnore | Case::OptionsAboveRepo => Some("analyzer:\n  errors:\n    unused_local_variable: ignore\n"),
        Case::Exclude => Some("analyzer:\n  exclude:\n    - gen/**\n"),
        Case::IncludeOutsideRepo => Some("include: ../shared.yaml\n"),
        _ => None,
    };
    if let Some(options) = options {
        let folder = if matches!(case, Case::OptionsAboveRepo) { &outer.0 } else { &repo };
        fs::write(folder.join("analysis_options.yaml"), options).unwrap();
    }
    let check = match case {
        Case::LintEnabled | Case::LintDisabled => "prefer_single_quotes",
        Case::Unknown => "no_such_code_xyz",
        _ => "unused_local_variable",
    };
    fs::write(repo.join("temperare.toml"), format!(
        "version = 1\n[[rule]]\nid = \"dart-rule\"\nengine = \"dart\"\ncheck = \"{check}\"\nfix = \"{FIX}\"\n"
    )).unwrap();
    let file = RuleFile::read(&repo.join("temperare.toml")).unwrap();
    let _absent_path = if matches!(case, Case::NoFilesAbsent | Case::Absent) {
        let guard = PathGuard(std::env::var_os("PATH"));
        let paths = std::env::split_paths(guard.0.as_ref().unwrap()).filter(|dir| {
            !dir.join("dart").metadata().is_ok_and(|m| m.is_file() && m.permissions().mode() & 0o111 != 0)
        });
        std::env::set_var("PATH", std::env::join_paths(paths).unwrap());
        Some(guard)
    } else { None };
    let runs = run_rules_versioned_at(&file, &repo, &[], None).runs;
    assert_eq!(runs.len(), 1, "{case:?}: {runs:?}");
    let run = &runs[0];
    match case {
        Case::Violation | Case::Clean | Case::LintEnabled | Case::NoFilesAbsent => {
            let RuleOutcome::Findings(findings) = &run.outcome else {
                panic!("{case:?}: expected Findings, got {:?}", run.outcome);
            };
            let count = usize::from(matches!(case, Case::Violation | Case::LintEnabled));
            assert_eq!(findings.len(), count, "{case:?}: {findings:?}");
            if count == 1 {
                assert_eq!(findings[0].path, RelPath::new("lib/a.dart"), "{case:?}");
            }
            if matches!(case, Case::Violation) {
                let finding = &findings[0];
                assert_eq!(finding.rule, "dart-rule");
                assert_eq!(finding.line(), Some(2));
                assert_eq!(finding.col, Some(7));
                assert_eq!(finding.what, "The value of the local variable 'unused' isn't used.");
                assert_eq!(finding.fix, FIX);
            }
            if matches!(case, Case::LintEnabled) {
                let finding = &findings[0];
                assert_eq!(finding.rule, "dart-rule");
                assert_eq!(finding.line(), Some(2));
                assert_eq!(finding.col, Some(9));
                assert_eq!(finding.what, "Unnecessary use of double quotes.");
                assert_eq!(finding.fix, FIX);
            }
            if matches!(case, Case::NoFilesAbsent) {
                assert!(run.no_files.is_some(), "{case:?}: {run:?}");
            }
        }
        Case::ErrorsIgnore | Case::Exclude | Case::OptionsAboveRepo
        | Case::GitignoredNestedOptions | Case::IncludeOutsideRepo
        | Case::EscapedIgnore | Case::MultilineQuoted | Case::EscapedInclude
        | Case::FlowErrors | Case::SymlinkedFolderOutside => {
            assert!(matches!(&run.outcome, RuleOutcome::Failed { .. }), "{case:?}: expected Failed, got {:?}", run.outcome);
            let RuleOutcome::Failed { stderr, .. } = &run.outcome else { unreachable!() };
            match case {
                Case::ErrorsIgnore => {
                    assert!(stderr.contains("analysis_options.yaml"), "{case:?}: {stderr}");
                    assert!(stderr.contains("ignore"), "{case:?}: {stderr}");
                }
                Case::Exclude => {
                    assert!(stderr.contains("analysis_options.yaml"), "{case:?}: {stderr}");
                    assert!(stderr.contains("exclude"), "{case:?}: {stderr}");
                }
                Case::OptionsAboveRepo => assert!(stderr.contains(outer.0.join("analysis_options.yaml").to_str().unwrap()), "{case:?}: {stderr}"),
                Case::GitignoredNestedOptions => assert!(stderr.contains("gen/analysis_options.yaml"), "{case:?}: {stderr}"),
                Case::IncludeOutsideRepo => assert!(stderr.contains("shared.yaml"), "{case:?}: {stderr}"),
                Case::EscapedIgnore | Case::MultilineQuoted | Case::FlowErrors => {
                    assert!(stderr.contains("analysis_options.yaml"), "{case:?}: {stderr}");
                    assert!(stderr.contains("unused_local_variable: ignore"), "{case:?}: {stderr}");
                    assert!(stderr.contains("dart hides findings"), "{case:?}: {stderr}");
                }
                Case::EscapedInclude => {
                    assert!(stderr.contains("ign.yaml"), "{case:?}: {stderr}");
                    assert!(stderr.contains("dart hides findings"), "{case:?}: {stderr}");
                }
                Case::SymlinkedFolderOutside => assert!(stderr.contains(fs::canonicalize(outer.0.join("outside/analysis_options.yaml")).unwrap().to_str().unwrap()), "{case:?}: {stderr}"),
                _ => unreachable!(),
            }
            assert!(stderr.contains("tm cannot turn it off; put exceptions in the rule file"), "{case:?}: {stderr}");
            assert!(!stderr.contains("S-67"), "{case:?}: {stderr}");
        }
        _ => {
            let RuleOutcome::NotAvailable { rule, engine, why } = &run.outcome else {
                panic!("{case:?}: expected NotAvailable, got {:?}", run.outcome);
            };
            assert_eq!(rule, "dart-rule");
            assert_eq!(*engine, Engine::Dart);
            match case {
                Case::LintDisabled => assert!(why.contains("prefer_single_quotes"), "{case:?}: {why}"),
                Case::Unknown => assert!(why.contains("no_such_code_xyz"), "{case:?}: {why}"),
                Case::NoPubspec => assert!(why.contains("pubspec.yaml"), "{case:?}: {why}"),
                Case::UnresolvedInclude | Case::UnfetchedPackage => assert!(why.contains("dart pub get"), "{case:?}: {why}"),
                Case::Absent => assert_eq!(why, "dart not found on PATH"),
                _ => unreachable!(),
            }
        }
    }
}

fn run_cases(cases: &[Case]) {
    let _lock = PATH_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let original = PathGuard(std::env::var_os("PATH"));
    let paths = std::iter::once(Path::new("/opt/homebrew/bin").to_path_buf())
        .chain(std::env::split_paths(original.0.as_ref().unwrap()));
    std::env::set_var("PATH", std::env::join_paths(paths).unwrap());
    let mut failed = Vec::new();
    for &case in cases {
        if std::panic::catch_unwind(|| check_case(case)).is_err() {
            eprintln!("{case:?}: FAILED");
            failed.push(case);
        } else {
            eprintln!("{case:?}: passed");
        }
    }
    assert!(failed.is_empty(), "failed rows: {failed:?}");
}

#[test]
fn dart_findings() {
    run_cases(&[Case::Violation, Case::Clean, Case::LintEnabled]);
}

#[test]
fn dart_not_available_and_settling() {
    run_cases(&[
        Case::LintDisabled, Case::Unknown, Case::NoPubspec, Case::UnresolvedInclude,
        Case::UnfetchedPackage, Case::NoFilesAbsent, Case::Absent,
    ]);
}

#[test]
fn dart_suppressions() {
    run_cases(&[
        Case::ErrorsIgnore, Case::Exclude, Case::OptionsAboveRepo,
        Case::GitignoredNestedOptions, Case::IncludeOutsideRepo,
        Case::EscapedIgnore, Case::MultilineQuoted, Case::EscapedInclude,
        Case::FlowErrors, Case::SymlinkedFolderOutside,
    ]);
}

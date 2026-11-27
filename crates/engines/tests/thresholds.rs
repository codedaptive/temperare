use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

use engines::{availability_in, run_rules_versioned_at, Availability};
use rules::{Engine, RelPath, RuleFile, RuleOutcome};

static NEXT_FOLDER: AtomicUsize = AtomicUsize::new(0);

struct TempFolder(PathBuf);

impl TempFolder {
    fn new(tag: &str) -> Self {
        let path = std::env::temp_dir().join(format!(
            "engines-thresholds-{}-{tag}-{}",
            std::process::id(),
            NEXT_FOLDER.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&path).unwrap();
        Self(fs::canonicalize(path).unwrap())
    }
}

impl Drop for TempFolder {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).unwrap();
    }
}

fn run(repo: &Path, engine: &str, check: &str, params: Option<&str>) -> RuleOutcome {
    let mut text = format!(
        "version = 1\n[[rule]]\nid = \"threshold\"\nengine = \"{engine}\"\ncheck = \"{check}\"\nfix = \"Reduce the value.\"\n"
    );
    if let Some(params) = params {
        text.push_str(&format!("[rule.params]\n{params}\n"));
    }
    let path = repo.join("temperare.toml");
    fs::write(&path, text).unwrap();
    let file = RuleFile::read(&path).unwrap();
    let mut runs = run_rules_versioned_at(&file, repo, &[], None).runs;
    assert_eq!(runs.len(), 1, "{runs:?}");
    let result = runs.remove(0);
    assert_eq!(result.rule, "threshold");
    result.outcome
}

fn assert_one_at(outcome: &RuleOutcome, path: &str, line: Option<u32>) {
    let RuleOutcome::Findings(findings) = outcome else { panic!("expected findings, got {outcome:?}") };
    assert_eq!(findings.len(), 1, "{findings:?}");
    assert_eq!(findings[0].path, RelPath::new(path));
    if let Some(line) = line {
        assert_eq!(findings[0].line(), Some(line));
    }
}

fn assert_clean(outcome: &RuleOutcome) {
    assert_eq!(*outcome, RuleOutcome::Findings(vec![]));
}

#[test]
fn ruff_complexity_threshold_is_per_run_and_preserves_repository_config() {
    let folder = TempFolder::new("ruff");
    assert_eq!(availability_in(Engine::Ruff, &folder.0), Availability::Available);
    fs::write(folder.0.join("example.py"), "def f(a):\n    if a == 1: return 1\n    elif a == 2: return 2\n    elif a == 3: return 3\n    return 0\n").unwrap();
    let config = folder.0.join("ruff.toml");
    fs::write(&config, "[lint.mccabe]\nmax-complexity = 50\n").unwrap();
    let original = fs::read(&config).unwrap();
    let low = run(&folder.0, "ruff", "C901", Some("max_complexity = 2"));
    let high = run(&folder.0, "ruff", "C901", Some("max_complexity = 5"));
    assert_eq!(fs::read(&config).unwrap(), original);
    assert_one_at(&low, "example.py", Some(1));
    assert_clean(&high);
}

#[test]
#[cfg_attr(windows, ignore = "SwiftLint publishes no Windows build")]
fn swiftlint_complexity_threshold_preserves_exclusions_and_repository_config() {
    let folder = TempFolder::new("swift-complexity");
    assert_eq!(availability_in(Engine::SwiftLint, &folder.0), Availability::Available);
    let source = "func f(_ a: Int) -> Int {\n    if a == 1 { return 1 }\n    if a == 2 { return 2 }\n    if a == 3 { return 3 }\n    if a == 4 { return 4 }\n    return 0\n}\n";
    fs::write(folder.0.join("Example.swift"), source).unwrap();
    fs::create_dir(folder.0.join("skipme")).unwrap();
    fs::write(folder.0.join("skipme/b.swift"), source).unwrap();
    let config = folder.0.join(".swiftlint.yml");
    fs::write(&config, "cyclomatic_complexity:\n  warning: 30\n  error: 40\nexcluded:\n  - skipme\n").unwrap();
    let original = fs::read(&config).unwrap();
    let low = run(&folder.0, "swiftlint", "cyclomatic_complexity", Some("max_complexity = 2"));
    let high = run(&folder.0, "swiftlint", "cyclomatic_complexity", Some("max_complexity = 10"));
    assert_eq!(fs::read(&config).unwrap(), original);
    assert_one_at(&low, "Example.swift", None);
    assert_clean(&high);
}

#[test]
#[cfg_attr(windows, ignore = "SwiftLint publishes no Windows build")]
fn swiftlint_file_length_threshold_changes_findings() {
    let folder = TempFolder::new("swift-length");
    assert_eq!(availability_in(Engine::SwiftLint, &folder.0), Availability::Available);
    fs::write(folder.0.join("Example.swift"), "let a = 1\nlet b = 2\nlet c = 3\nlet d = 4\nlet e = 5\nlet f = 6\nlet g = 7\n").unwrap();
    let low = run(&folder.0, "swiftlint", "file_length", Some("max_lines = 5"));
    let high = run(&folder.0, "swiftlint", "file_length", Some("max_lines = 100"));
    assert_one_at(&low, "Example.swift", None);
    assert_clean(&high);
}

#[test]
fn clippy_nesting_threshold_is_per_run_even_with_cargo_cache() {
    let folder = TempFolder::new("clippy");
    assert_eq!(availability_in(Engine::Clippy, &folder.0), Availability::Available);
    fs::create_dir(folder.0.join("src")).unwrap();
    fs::write(folder.0.join("Cargo.toml"), "[package]\nname = \"threshold_zit\"\nversion = \"0.1.0\"\nedition = \"2021\"\n[workspace]\n").unwrap();
    fs::write(folder.0.join("src/lib.rs"), "pub fn f(a: bool, b: bool, c: bool, d: bool) {\n    if a {\n        if b {\n            if c {\n                if d {\n                    println!(\"nested\");\n                }\n            }\n        }\n    }\n}\n").unwrap();
    let config = folder.0.join("clippy.toml");
    fs::write(&config, "excessive-nesting-threshold = 50\n").unwrap();
    let original = fs::read(&config).unwrap();
    let low = run(&folder.0, "clippy", "clippy::excessive_nesting", Some("max_nesting = 2"));
    let default = run(&folder.0, "clippy", "clippy::excessive_nesting", None);
    let low_again = run(&folder.0, "clippy", "clippy::excessive_nesting", Some("max_nesting = 2"));
    let high = run(&folder.0, "clippy", "clippy::excessive_nesting", Some("max_nesting = 10"));
    assert_eq!(fs::read(&config).unwrap(), original);
    assert_one_at(&low, "src/lib.rs", None);
    assert_clean(&default);
    assert_one_at(&low_again, "src/lib.rs", None);
    assert_clean(&high);
}

#[test]
fn ruff_rejects_zero_and_string_complexity_thresholds_as_failed() {
    let folder = TempFolder::new("ruff-bad");
    assert_eq!(availability_in(Engine::Ruff, &folder.0), Availability::Available);
    fs::write(folder.0.join("example.py"), "def f():\n    return 0\n").unwrap();
    for value in ["0", "\"5\""] {
        let outcome = run(&folder.0, "ruff", "C901", Some(&format!("max_complexity = {value}")));
        let RuleOutcome::Failed { rule, engine, stderr, .. } = outcome else {
            panic!("{value}: expected Failed, got {outcome:?}");
        };
        assert_eq!(rule, "threshold");
        assert_eq!(engine, Engine::Ruff);
        assert!(stderr.contains("max_complexity"), "{value}: {stderr}");
    }
}

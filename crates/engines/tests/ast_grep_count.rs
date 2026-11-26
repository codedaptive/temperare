use std::fs;
use std::path::PathBuf;

use engines::{availability_in, run_rules_versioned_at, Availability};
use rules::{Engine, RelPath, RuleFile, RuleOutcome};

#[path = "support/git.rs"]
mod git_support;

const YAML: &str = r#"id: root
language: Rust
message: root
severity: warning
rule:
  kind: source_file
---
id: test-function
language: Rust
message: test function
severity: warning
rule:
  kind: function_item
  follows:
    kind: attribute_item
    has:
      kind: attribute
      regex: ^test$
"#;

const RULE: &str = r#"version = 1
[[rule]]
id = "setup-count"
engine = "ast-grep"
check = "setup.yml"
paths = ["tests/**"]
fix = "Reduce setup."
params = { count = "setup-vs-tests", shared_setup = ["tests/common/**"] }
"#;

const BALANCED: &str = "const A: i32 = 1;\n#[test]\nfn works() {\n    assert_eq!(A, 1);\n}\n";
const CHANGED: &str = "const A: i32 = 1;\nconst B: i32 = 2;\nconst C: i32 = 3;\n#[test]\nfn works() {\n    assert_eq!(A, 1);\n}\n";
const HEAVY: &str = "const A: i32 = 1;\nconst B: i32 = 2;\nconst C: i32 = 3;\nconst D: i32 = 4;\n#[test]\nfn works() {\n    assert_eq!(A, 1);\n}\n";

struct Repo(PathBuf);

impl Repo {
    fn new(tag: &str) -> (Self, String) {
        let path = std::env::temp_dir().join(format!("engines-zit-{}-{tag}", std::process::id()));
        fs::create_dir(&path).unwrap();
        let repo = Self(fs::canonicalize(path).unwrap());
        git_support::git(&repo.0, &["init", "-q"], b"");
        fs::create_dir_all(repo.0.join("tests/common")).unwrap();
        fs::write(repo.0.join("temperare.toml"), RULE).unwrap();
        fs::write(repo.0.join("setup.yml"), YAML).unwrap();
        fs::write(repo.0.join("tests/a_tests.rs"), BALANCED).unwrap();
        fs::write(repo.0.join("tests/b_tests.rs"), HEAVY).unwrap();
        fs::write(repo.0.join("tests/common/mod.rs"), HEAVY).unwrap();
        git_support::git(&repo.0, &["add", "."], b"");
        git_support::git(&repo.0, &["-c", "user.name=t", "-c", "user.email=t@t", "commit", "-qm", "base"], b"");
        let base = String::from_utf8(git_support::git(&repo.0, &["rev-parse", "HEAD"], b"")).unwrap().trim().to_string();
        (repo, base)
    }

    fn run(&self, base: Option<&str>) -> Vec<RuleOutcome> {
        let file = RuleFile::read(&self.0.join("temperare.toml")).unwrap();
        run_rules_versioned_at(&file, &self.0, &[], base).runs.into_iter().map(|run| run.outcome).collect()
    }
}

impl Drop for Repo {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).unwrap();
    }
}

fn assert_finding(outcomes: Vec<RuleOutcome>, path: &str, line: u32, what: &str) {
    assert_eq!(outcomes.len(), 1, "{outcomes:?}");
    let RuleOutcome::Findings(findings) = &outcomes[0] else {
        panic!("expected findings, got {:?}", outcomes[0]);
    };
    assert_eq!(findings.len(), 1, "{findings:?}");
    assert_eq!(findings[0].path, RelPath::new(path));
    assert_eq!(findings[0].line(), Some(line));
    assert_eq!(findings[0].col, None);
    assert_eq!(findings[0].what, what);
    assert_eq!(findings[0].fix, "Reduce setup.");
}

#[test]
fn base_reports_only_changed_non_shared_heavy_test_file() {
    let (repo, base) = Repo::new("count-changed");
    assert_eq!(availability_in(Engine::AstGrep, &repo.0), Availability::Available);
    fs::write(repo.0.join("tests/a_tests.rs"), CHANGED).unwrap();
    fs::write(repo.0.join("tests/common/mod.rs"), format!("{HEAVY}const E: i32 = 5;\n")).unwrap();
    let outcomes = repo.run(Some(&base));
    drop(repo);
    // The attribute is outside the function match: 3 constants + 1 attribute setup, 3 test lines.
    assert_finding(outcomes, "tests/a_tests.rs", 2, "4 lines of setup, 3 lines of tests");
}

#[test]
fn base_reports_untracked_heavy_test_file_at_line_one() {
    let (repo, base) = Repo::new("count-untracked");
    assert_eq!(availability_in(Engine::AstGrep, &repo.0), Availability::Available);
    fs::write(repo.0.join("tests/new_tests.rs"), HEAVY).unwrap();
    let outcomes = repo.run(Some(&base));
    drop(repo);
    // Four constants and the attribute are setup; the three function lines are tests.
    assert_finding(outcomes, "tests/new_tests.rs", 1, "5 lines of setup, 3 lines of tests");
}

#[test]
fn count_without_base_is_not_available() {
    let (repo, _base) = Repo::new("count-no-base");
    assert_eq!(availability_in(Engine::AstGrep, &repo.0), Availability::Available);
    fs::write(repo.0.join("tests/a_tests.rs"), CHANGED).unwrap();
    let outcomes = repo.run(None);
    drop(repo);
    assert_eq!(outcomes, vec![RuleOutcome::NotAvailable {
        rule: "setup-count".to_string(),
        engine: Engine::AstGrep,
        why: "reports only what a change adds; run with --base".to_string(),
    }]);
}

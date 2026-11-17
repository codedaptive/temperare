//! The report's new-only filter against repositories made for these tests.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use report::build_report;
use rules::{Engine, Exception, Finding, Kind, RelPath, RuleFile, RuleOutcome, Severity};

struct Scratch {
    dir: PathBuf,
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.dir);
    }
}

fn git(dir: &Path, args: &[&str]) -> Vec<u8> {
    let mut command = Command::new("git");
    command.arg("-C").arg(dir).args(args);
    for (key, _) in std::env::vars_os() {
        if key.to_string_lossy().starts_with("GIT_") {
            command.env_remove(key);
        }
    }
    let output = command
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .output()
        .unwrap();
    assert!(output.status.success(), "git {args:?}: {}", String::from_utf8_lossy(&output.stderr));
    output.stdout
}

fn repo(name: &str, initial_file: &str) -> (Scratch, procio::Oid) {
    let dir = std::env::temp_dir().join(format!("report-zit-{}-{name}", std::process::id()));
    fs::create_dir(&dir).unwrap();
    let scratch = Scratch { dir };
    git(&scratch.dir, &["init", "-q", "-b", "main"]);
    git(&scratch.dir, &["config", "user.name", "report zit"]);
    git(&scratch.dir, &["config", "user.email", "zit@report.invalid"]);
    git(&scratch.dir, &["config", "commit.gpgsign", "false"]);
    write(&scratch.dir, initial_file, b"base\n");
    git(&scratch.dir, &["add", "-A"]);
    git(&scratch.dir, &["commit", "-q", "-m", "base"]);
    let base = procio::rev_parse(&scratch.dir, "HEAD").unwrap().unwrap();
    (scratch, base)
}

fn write(dir: &Path, path: &str, contents: &[u8]) {
    let file = dir.join(path);
    fs::create_dir_all(file.parent().unwrap()).unwrap();
    fs::write(file, contents).unwrap();
}

fn rule_file(folder: &Path, exceptions: Vec<Exception>) -> RuleFile {
    RuleFile { folder: folder.to_path_buf(), kits: Vec::new(), owner: None, rules: Vec::new(), exceptions }
}

fn finding(rule: &str, path: &str, line: u32) -> Finding {
    Finding {
        rule: rule.to_string(),
        engine: Engine::Clippy,
        kind: Kind::Quality,
        severity: Severity::Error,
        owner: None,
        path: RelPath::new(path),
        line,
        col: Some(9),
        what: "violation".to_string(),
        fix: "Fix the violation.".to_string(),
    }
}

fn exception(rule: &str, path: &str, reason: &str) -> Exception {
    Exception {
        rule: rule.to_string(),
        path: RelPath::new(path),
        what: None,
        reason: reason.to_string(),
        approved: "approved".to_string(),
    }
}

fn outcomes(findings: Vec<Finding>) -> Vec<(String, RuleOutcome)> {
    vec![("test-rule".to_string(), RuleOutcome::Findings(findings))]
}

#[test]
fn new_lines_untracked_files_and_nested_repositories_are_kept() {
    let (repo, base) = repo("new_lines_untracked_nested", "a.rs");
    write(&repo.dir, "a.rs", b"base\nadded\n");
    write(&repo.dir, "b.rs", b"new\n");
    fs::create_dir(repo.dir.join("nested")).unwrap();
    git(&repo.dir.join("nested"), &["init", "-q", "-b", "main"]);
    write(&repo.dir, "nested/c.rs", b"1\n2\n3\n4\n5\n6\n7\n");
    assert_eq!(
        git(&repo.dir, &["ls-files", "--others", "--exclude-standard", "-z", "--full-name", "--", ":/"]),
        b"b.rs\0nested/\0"
    );

    let findings = vec![
        finding("test-rule", "nested/c.rs", 7),
        finding("test-rule", "a.rs", 1),
        finding("test-rule", "b.rs", 1),
        finding("test-rule", "a.rs", 2),
    ];
    let report = build_report(&rule_file(&repo.dir, vec![]), outcomes(findings), Some((&repo.dir, &base))).unwrap();
    let locations: Vec<_> = report.findings.iter().map(|f| (f.path.as_str(), f.line)).collect();
    assert_eq!(locations, vec![("a.rs", 2), ("b.rs", 1), ("nested/c.rs", 7)]);
    assert_eq!(report.counts.findings, 3);
    assert!(report.silenced.is_empty());
    assert_eq!(report.base, Some(base));
}

#[test]
fn base_filter_precedes_exception_matching() {
    let (repo, base) = repo("filter_before_exceptions", "a.rs");
    write(&repo.dir, "a.rs", b"base\nadded\n");
    let rules = rule_file(&repo.dir, vec![exception("old-rule", "a.rs", "old"), exception("new-rule", "a.rs", "new")]);
    let old_only = build_report(
        &rules,
        outcomes(vec![finding("old-rule", "a.rs", 1)]),
        Some((&repo.dir, &base)),
    )
    .unwrap();
    assert!(old_only.findings.is_empty());
    assert!(old_only.silenced.is_empty());
    assert_eq!(old_only.counts.silenced, 0);

    let report = build_report(
        &rules,
        outcomes(vec![finding("old-rule", "a.rs", 1), finding("new-rule", "a.rs", 2)]),
        Some((&repo.dir, &base)),
    )
    .unwrap();
    assert!(report.findings.is_empty());
    assert_eq!(report.counts.findings, 0);
    assert_eq!(report.counts.silenced, 1);
    assert_eq!(report.silenced.len(), 1);
    assert_eq!(report.silenced[0].finding.path.as_str(), "a.rs");
    assert_eq!(report.silenced[0].finding.line, 2);
    assert_eq!(report.silenced[0].finding.rule, "new-rule");
    assert_eq!(report.silenced[0].exception.reason, "new");
}

#[test]
fn subfolder_findings_are_rerooted_for_filtering_but_keep_their_paths() {
    let (repo, base) = repo("subfolder_reroot", "sub/x.rs");
    write(&repo.dir, "sub/x.rs", b"base\nadded\n");
    let report = build_report(
        &rule_file(&repo.dir.join("sub"), vec![]),
        outcomes(vec![finding("test-rule", "x.rs", 1), finding("test-rule", "x.rs", 2)]),
        Some((&repo.dir, &base)),
    )
    .unwrap();
    let locations: Vec<_> = report.findings.iter().map(|f| (f.path.as_str(), f.line)).collect();
    assert_eq!(locations, vec![("x.rs", 2)]);
    assert_eq!(report.counts.findings, 1);
    assert!(report.silenced.is_empty());
}

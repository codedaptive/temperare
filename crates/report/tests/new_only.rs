//! The report's new-only filter, and a flagged rule's place with a base, against repositories made for these tests.

mod common;

use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use ccvcslib::{GitVcs, Vcs};
use report::{build_report_with_no_files, CoversNothing};
use rules::{Engine, Exception, Finding, Kind, RelPath, Rule, RuleFile, RuleOutcome, Severity};

struct Scratch {
    dir: PathBuf,
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.dir);
    }
}

fn git(dir: &Path, args: &[&str]) -> Vec<u8> {
    let output = procio::run(&procio::Cmd {
        args: args.iter().map(OsString::from).collect(),
        cwd: Some(dir.to_path_buf()),
        env: vec![
            (OsString::from("GIT_CONFIG_GLOBAL"), OsString::from("/dev/null")),
            (OsString::from("GIT_CONFIG_NOSYSTEM"), OsString::from("1")),
        ],
        env_remove: std::env::vars_os()
            .map(|(key, _)| key)
            .filter(|key| key.to_string_lossy().starts_with("GIT_"))
            .collect(),
        ..procio::Cmd::new("git")
    })
    .unwrap();
    assert!(output.code == Some(0), "git {args:?}: {}", String::from_utf8_lossy(&output.stderr));
    output.stdout
}

fn repo(name: &str, initial_file: &str) -> (Scratch, report::Oid) {
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
    let rev = GitVcs::default().resolve_revision(&scratch.dir, "HEAD").unwrap().unwrap();
    let base = report::Oid::parse(&rev.0).unwrap();
    (scratch, base)
}

fn write(dir: &Path, path: &str, contents: &[u8]) {
    let file = dir.join(path);
    fs::create_dir_all(file.parent().unwrap()).unwrap();
    fs::write(file, contents).unwrap();
}

fn rule_file(folder: &Path, exceptions: Vec<Exception>) -> RuleFile {
    let mut rules = RuleFile::new(folder.to_path_buf());
    rules.exceptions = exceptions;
    rules
}

fn finding(rule: &str, path: &str, line: impl Into<Option<u32>>) -> Finding {
    Finding::new(
        rule,
        (Engine::Clippy, Kind::Quality, Severity::Error),
        RelPath::new(path),
        line,
        Some(9),
        "violation",
        "Fix the violation.",
    )
}

fn exception(rule: &str, path: &str, reason: &str) -> Exception {
    Exception::new(rule, path, reason, "approved")
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
    let entries = GitVcs::default().untracked_entries(&repo.dir).unwrap();
    let paths: Vec<_> = entries.iter().map(|path| path.as_os_str().as_encoded_bytes()).collect();
    assert_eq!(paths, vec![b"b.rs".as_slice(), b"nested/".as_slice()]);

    let findings = vec![
        finding("test-rule", "nested/c.rs", 7),
        finding("test-rule", "a.rs", 1),
        finding("test-rule", "b.rs", 1),
        finding("test-rule", "a.rs", 2),
    ];
    let before = SystemTime::now();
    let report = build_report_with_no_files(
        &rule_file(&repo.dir, vec![]),
        outcomes(findings),
        Some((&repo.dir, &base)),
        Vec::new(),
    )
    .unwrap();
    let after = SystemTime::now();
    assert!(report.date == common::utc_date(before) || report.date == common::utc_date(after), "{}", report.date);
    let locations: Vec<_> = report.findings.iter().map(|f| (f.path.as_str(), f.line())).collect();
    assert_eq!(locations, vec![("a.rs", Some(2)), ("b.rs", Some(1)), ("nested/c.rs", Some(7))]);
    assert_eq!(report.counts.findings, 3);
    assert!(report.silenced.is_empty());
    assert_eq!(report.base, Some(base));
}

#[test]
fn subfolder_repository_argument_filters_the_whole_working_copy() {
    let (repo, base) = repo("subfolder_repository", "sub/x.rs");
    write(&repo.dir, "sub/x.rs", b"base\nadded\n");
    write(&repo.dir, "b.rs", b"new\n");
    write(&repo.dir, "sub/y.rs", b"new\n");
    let report = build_report_with_no_files(
        &rule_file(&repo.dir, vec![]),
        outcomes(vec![
            finding("test-rule", "sub/x.rs", 1),
            finding("test-rule", "sub/x.rs", 2),
            finding("test-rule", "b.rs", 1),
            finding("test-rule", "sub/y.rs", 1),
        ]),
        Some((&repo.dir.join("sub"), &base)),
        Vec::new(),
    )
    .unwrap();
    let locations: Vec<_> = report.findings.iter().map(|f| (f.path.as_str(), f.line())).collect();
    assert_eq!(locations, vec![("b.rs", Some(1)), ("sub/x.rs", Some(2)), ("sub/y.rs", Some(1))]);
}

#[test]
fn base_filter_precedes_exception_matching() {
    let (repo, base) = repo("filter_before_exceptions", "a.rs");
    write(&repo.dir, "a.rs", b"base\nadded\n");
    let rules = rule_file(&repo.dir, vec![exception("old-rule", "a.rs", "old"), exception("new-rule", "a.rs", "new")]);
    let old_only = build_report_with_no_files(
        &rules,
        outcomes(vec![finding("old-rule", "a.rs", 1)]),
        Some((&repo.dir, &base)),
        Vec::new(),
    )
    .unwrap();
    assert!(old_only.findings.is_empty());
    assert!(old_only.silenced.is_empty());
    assert!(old_only.silenced_by_rule.is_empty());
    assert_eq!(old_only.counts.silenced, 0);

    let report = build_report_with_no_files(
        &rules,
        outcomes(vec![finding("old-rule", "a.rs", 1), finding("new-rule", "a.rs", 2)]),
        Some((&repo.dir, &base)),
        Vec::new(),
    )
    .unwrap();
    assert!(report.findings.is_empty());
    assert_eq!(report.counts.findings, 0);
    assert_eq!(report.counts.silenced, 1);
    assert_eq!(report.silenced_by_rule, vec![report::RuleCount { rule: "new-rule".to_string(), count: 1 }]);
    assert_eq!(report.silenced.len(), 1);
    assert_eq!(report.silenced[0].finding.path.as_str(), "a.rs");
    assert_eq!(report.silenced[0].finding.line(), Some(2));
    assert_eq!(report.silenced[0].finding.rule, "new-rule");
    assert_eq!(report.silenced[0].exception.reason, "new");
}

#[test]
fn subfolder_findings_are_rerooted_for_filtering_but_keep_their_paths() {
    let (repo, base) = repo("subfolder_reroot", "sub/x.rs");
    write(&repo.dir, "sub/x.rs", b"base\nadded\n");
    let report = build_report_with_no_files(
        &rule_file(&repo.dir.join("sub"), vec![]),
        outcomes(vec![finding("test-rule", "x.rs", 1), finding("test-rule", "x.rs", 2)]),
        Some((&repo.dir, &base)),
        Vec::new(),
    )
    .unwrap();
    let locations: Vec<_> = report.findings.iter().map(|f| (f.path.as_str(), f.line())).collect();
    assert_eq!(locations, vec![("x.rs", Some(2))]);
    assert_eq!(report.counts.findings, 1);
    assert!(report.silenced.is_empty());
}

#[test]
fn a_flagged_own_rule_with_a_base_covers_nothing_and_its_finding_is_dropped() {
    let (repo, base) = repo("flagged_own_rule_with_base", "a.rs");
    write(&repo.dir, "a.rs", b"base\nadded\n");
    let mut rules = rule_file(&repo.dir, vec![]);
    rules.rules = vec![Rule::new("own-x", Engine::Clippy, "lint")];
    let report = build_report_with_no_files(
        &rules,
        outcomes(vec![finding("own-x", "a.rs", 2), finding("test-rule", "a.rs", 2)]),
        Some((&repo.dir, &base)),
        vec![("own-x".to_string(), "its paths match no file in the rule file's folder".to_string())],
    )
    .unwrap();
    assert_eq!(report.covers_nothing, vec![CoversNothing { rule: "own-x".into(), engine: Engine::Clippy }]);
    let kept: Vec<_> = report.findings.iter().map(|f| (f.path.as_str(), f.line(), f.rule.as_str())).collect();
    assert_eq!(kept, vec![("a.rs", Some(2), "test-rule")]);
    assert_eq!(report.counts.covers_nothing, 1);
    assert_eq!(report.counts.findings, 1);
    assert!(report.not_applicable.is_empty());
}

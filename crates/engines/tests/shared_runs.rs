use std::fs;
use std::path::{Path, PathBuf};

use engines::{availability, run_rules_versioned, Availability};
use rules::{Engine, RelPath, RuleFile, RuleOutcome};

struct TempFolder(PathBuf);

impl TempFolder {
    fn new(tag: &str) -> Self {
        let path = std::env::temp_dir().join(format!("engines-shared-runs-{}-{tag}", std::process::id()));
        fs::create_dir(&path).unwrap();
        Self(fs::canonicalize(path).unwrap())
    }
}

impl Drop for TempFolder {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).unwrap();
    }
}

fn rule(id: &str, engine: &str, check: &str, paths: Option<&str>) -> String {
    let paths = paths.map_or(String::new(), |path| format!("paths = [\"{path}\"]\n"));
    format!("[[rule]]\nid = \"{id}\"\nengine = \"{engine}\"\ncheck = \"{check}\"\nfix = \"Fix it.\"\n{paths}")
}

// Engines that lint files in parallel print findings in no fixed order.
fn sorted_outcome(outcome: &RuleOutcome) -> RuleOutcome {
    let mut outcome = outcome.clone();
    if let RuleOutcome::Findings(findings) = &mut outcome {
        findings.sort_by(|a, b| (&a.path, a.line(), a.col, &a.what).cmp(&(&b.path, b.line(), b.col, &b.what)));
    }
    outcome
}

fn assert_shared(repo: &Path, rules: &[String; 3], kept: &str, excluded: &str) {
    let file = RuleFile::parse(&format!("version = 1\n{}", rules.concat()), repo).unwrap();
    let grouped = run_rules_versioned(&file, repo, &[]).runs;
    assert_eq!(grouped.len(), 3);
    for (index, rule) in rules.iter().enumerate() {
        let file = RuleFile::parse(&format!("version = 1\n{rule}"), repo).unwrap();
        let alone = run_rules_versioned(&file, repo, &[]).runs;
        assert_eq!(alone.len(), 1);
        assert_eq!(grouped[index].rule, file.rules[0].id);
        assert_eq!(grouped[index].rule, alone[0].rule);
        assert_eq!(sorted_outcome(&grouped[index].outcome), sorted_outcome(&alone[0].outcome), "{}", grouped[index].rule);
    }
    for run in &grouped {
        let RuleOutcome::Findings(findings) = &run.outcome else {
            panic!("{} did not produce findings: {:?}", run.rule, run.outcome);
        };
        assert!(!findings.is_empty(), "{} found nothing", run.rule);
    }
    let RuleOutcome::Findings(all) = &grouped[0].outcome else { unreachable!() };
    assert!(all.iter().any(|finding| finding.path == RelPath::new(excluded)), "{all:?}");
    let RuleOutcome::Findings(limited) = &grouped[2].outcome else { unreachable!() };
    assert!(limited.iter().all(|finding| finding.path == RelPath::new(kept)), "{limited:?}");
    assert_eq!(grouped.iter().filter(|run| run.raw_chars.is_some()).count(), 1, "shared check runs: {grouped:?}");
}

fn assert_incompatible_ruff_runs(repo: &Path, rules: &[String; 2]) {
    let file = RuleFile::parse(&format!("version = 1\n{}", rules.concat()), repo).unwrap();
    let grouped = run_rules_versioned(&file, repo, &[]).runs;
    assert_eq!(grouped.len(), 2);
    for (index, rule) in rules.iter().enumerate() {
        let file = RuleFile::parse(&format!("version = 1\n{rule}"), repo).unwrap();
        let alone = run_rules_versioned(&file, repo, &[]).runs;
        assert_eq!(alone.len(), 1);
        assert_eq!(grouped[index].rule, file.rules[0].id);
        assert_eq!(grouped[index].rule, alone[0].rule);
        assert_eq!(sorted_outcome(&grouped[index].outcome), sorted_outcome(&alone[0].outcome), "{}", grouped[index].rule);
    }
    let RuleOutcome::Findings(findings) = &grouped[0].outcome else {
        panic!("D203 did not produce findings: {:?}", grouped[0].outcome);
    };
    assert!(!findings.is_empty(), "D203 found nothing");
    assert_eq!(grouped.iter().filter(|run| run.raw_chars.is_some()).count(), 2, "separate checks: {grouped:?}");
}

#[test]
fn ruff_incompatible_rules_match_separate_runs() {
    assert_eq!(availability(Engine::Ruff), Availability::Available);
    let repo = TempFolder::new("ruff-incompatible");
    fs::write(repo.0.join("example.py"), "class A:\n    \"\"\"Doc.\"\"\"\n").unwrap();
    let rules = [rule("d203", "ruff", "D203", None), rule("d211", "ruff", "D211", None)];
    assert_incompatible_ruff_runs(&repo.0, &rules);
}

#[test]
fn ruff_shared_runs_match_separate_rules() {
    assert_eq!(availability(Engine::Ruff), Availability::Available);
    let repo = TempFolder::new("ruff");
    let source = "import os\nvalue = None\nif value == None:\n    pass\n";
    fs::write(repo.0.join("keep.py"), source).unwrap();
    fs::write(repo.0.join("omit.py"), source).unwrap();
    let rules = [
        rule("none", "ruff", "E711", None),
        rule("import", "ruff", "F401", None),
        rule("limited", "ruff", "E711", Some("keep.py")),
    ];
    assert_shared(&repo.0, &rules, "keep.py", "omit.py");
}

#[test]
fn swiftlint_shared_runs_match_separate_rules() {
    assert_eq!(availability(Engine::SwiftLint), Availability::Available);
    let repo = TempFolder::new("swiftlint");
    let source = "func example(_ value: Any) throws {\n    let text = value as! String\n    let result = try! throwing()\n    print(text, result)\n}\nfunc throwing() throws -> Int { 1 }\n";
    fs::write(repo.0.join("Keep.swift"), source).unwrap();
    fs::write(repo.0.join("Omit.swift"), source).unwrap();
    let rules = [
        rule("cast", "swiftlint", "force_cast", None),
        rule("try", "swiftlint", "force_try", None),
        rule("limited", "swiftlint", "force_cast", Some("Keep.swift")),
    ];
    assert_shared(&repo.0, &rules, "Keep.swift", "Omit.swift");
}

#[test]
fn clippy_shared_runs_match_separate_rules() {
    assert_eq!(availability(Engine::Clippy), Availability::Available);
    let repo = TempFolder::new("clippy");
    fs::create_dir(repo.0.join("src")).unwrap();
    fs::create_dir(repo.0.join(".cargo")).unwrap();
    fs::write(repo.0.join(".cargo/config.toml"), "[net]\noffline = true\n").unwrap();
    fs::write(repo.0.join("Cargo.toml"), "[package]\nname = \"shared_runs_zit\"\nversion = \"0.1.0\"\nedition = \"2021\"\n[workspace]\n").unwrap();
    let source = "pub fn compare(value: bool) -> bool {\n    return value == true;\n}\n";
    fs::write(repo.0.join("src/lib.rs"), format!("pub mod other;\n{source}")).unwrap();
    fs::write(repo.0.join("src/other.rs"), source).unwrap();
    let rules = [
        rule("bool", "clippy", "clippy::bool_comparison", None),
        rule("return", "clippy", "clippy::needless_return", None),
        rule("limited", "clippy", "clippy::bool_comparison", Some("src/lib.rs")),
    ];
    assert_shared(&repo.0, &rules, "src/lib.rs", "src/other.rs");
}

#[test]
fn ast_grep_shared_runs_match_separate_rules() {
    assert_eq!(availability(Engine::AstGrep), Availability::Available);
    let repo = TempFolder::new("ast-grep");
    let source = "console.log(foo);\nconsole.log(bar);\n";
    fs::write(repo.0.join("keep.js"), source).unwrap();
    fs::write(repo.0.join("omit.js"), source).unwrap();
    fs::write(repo.0.join("names.yml"), "id: names\nlanguage: JavaScript\nmessage: matched name\nseverity: warning\nrule:\n  kind: identifier\n  regex: '^(foo|bar)$'\n").unwrap();
    let rules = [
        rule("all", "ast-grep", "names.yml", Some("*.js")),
        rule("other", "ast-grep", "names.yml", Some("omit.js")),
        rule("limited", "ast-grep", "names.yml", Some("keep.js")),
    ];
    assert_shared(&repo.0, &rules, "keep.js", "omit.js");
}

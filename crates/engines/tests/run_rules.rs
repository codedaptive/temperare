use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

use engines::{availability, run_rules_counted, Availability};
use rules::{Engine, Kind, RelPath, RuleFile, RuleOutcome, Severity};

/// Each of `run_rules_counted`'s runs read as (rule, outcome).
fn outcomes_of(file: &RuleFile, repo: &Path, only: &[RelPath]) -> Vec<(String, RuleOutcome)> {
    run_rules_counted(file, repo, only).into_iter().map(|run| (run.rule, run.outcome)).collect()
}

static NEXT_FOLDER: AtomicUsize = AtomicUsize::new(0);

struct TempFolder(PathBuf);

impl TempFolder {
    fn new(tag: &str) -> Self {
        let path = std::env::temp_dir().join(format!(
            "engines-run-rules-{}-{tag}-{}",
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

const RULES: &str = r#"version = 1
owner = "fallback"

[kits.rust]
paths = ["src/**"]
owner = "rust-team"
[kits.swift]
paths = ["*.swift"]
owner = "swift-team"
[kits.python]
paths = ["*.py"]
owner = "python-team"
[kits.javascript]
paths = ["*.js"]
owner = "javascript-team"

[[rule]]
id = "rust-bool"
engine = "clippy"
kind = "quality"
check = "clippy::bool_comparison"
severity = "warning"
fix = "Simplify the comparison."

[[rule]]
id = "swift-cast"
engine = "swiftlint"
kind = "security"
check = "force_cast"
severity = "error"
fix = "Use a checked cast."

[[rule]]
id = "python-none"
engine = "ruff"
kind = "quality"
check = "E711"
severity = "error"
fix = "Use identity for None."

[[rule]]
id = "js-names"
engine = "ast-grep"
kind = "security"
check = "names.yml"
severity = "warning"
fix = "Rename the identifier."
params = { names = ["foo", "bar"] }

[[rule]]
id = "tessera-slop"
engine = "tessera"
kind = "quality"
check = "slop"
severity = "warning"
fix = "Remove slop."
"#;

#[test]
fn run_rules_reports_ordered_findings_and_filters_to_one_file() {
    for engine in [Engine::Clippy, Engine::SwiftLint, Engine::Ruff, Engine::AstGrep] {
        assert_eq!(availability(engine), Availability::Available, "{engine:?}");
    }
    let folder = TempFolder::new("all");
    fs::create_dir(folder.0.join("src")).unwrap();
    fs::write(folder.0.join("Cargo.toml"), "[package]\nname = \"run_rules_zit\"\nversion = \"0.1.0\"\nedition = \"2021\"\n[workspace]\n").unwrap();
    fs::write(folder.0.join("src/lib.rs"), "pub fn compare(value: bool) -> bool {\n    value == true\n}\n").unwrap();
    fs::write(folder.0.join("Example.swift"), "func example(_ value: Any) {\n    let text = value as! String\n    print(text)\n}\n").unwrap();
    fs::write(folder.0.join("example.py"), "value = None\nif value == None:\n    pass\n").unwrap();
    fs::write(folder.0.join("example.js"), "console.log(foo);\nconsole.log(baz);\n").unwrap();
    fs::write(folder.0.join("names.yml"), "id: selected-names\nlanguage: JavaScript\nmessage: matched name\nseverity: warning\nrule:\n  kind: identifier\n  regex: '^(?:{{names}})$'\n").unwrap();
    fs::write(folder.0.join("temperare.toml"), RULES).unwrap();
    let file = RuleFile::read(&folder.0.join("temperare.toml")).unwrap();
    let outcomes = outcomes_of(&file, &folder.0, &[]);
    let filtered = outcomes_of(&file, &folder.0, &[RelPath::new("example.py")]);
    let cache_exists = folder.0.join(".ruff_cache").exists();
    drop(folder);

    assert!(!cache_exists, "ruff left a .ruff_cache directory");
    let ids: Vec<_> = outcomes.iter().map(|(id, _)| id.as_str()).collect();
    assert_eq!(ids, ["rust-bool", "swift-cast", "python-none", "js-names", "tessera-slop"]);
    let expected = [
        ("rust-bool", Engine::Clippy, "src/lib.rs", 2, "true", Kind::Quality, Severity::Warning, "Simplify the comparison.", "rust-team"),
        ("swift-cast", Engine::SwiftLint, "Example.swift", 2, "cast", Kind::Security, Severity::Error, "Use a checked cast.", "swift-team"),
        ("python-none", Engine::Ruff, "example.py", 2, "None", Kind::Quality, Severity::Error, "Use identity for None.", "python-team"),
        ("js-names", Engine::AstGrep, "example.js", 1, "matched name", Kind::Security, Severity::Warning, "Rename the identifier.", "javascript-team"),
    ];
    for ((_, outcome), (id, engine, path, line, message, kind, severity, fix, owner)) in outcomes.iter().zip(expected) {
        let RuleOutcome::Findings(findings) = outcome else { panic!("{id}: {outcome:?}") };
        assert_eq!(findings.len(), 1, "{id}: {findings:?}");
        let hit = &findings[0];
        assert_eq!(hit.rule, id);
        assert_eq!(hit.engine, engine);
        assert_eq!(hit.path, RelPath::new(path));
        assert_eq!(hit.line(), Some(line));
        assert!(hit.what.to_lowercase().contains(&message.to_lowercase()), "{hit:?}");
        assert_eq!(hit.kind, kind);
        assert_eq!(hit.severity, severity);
        assert_eq!(hit.fix, fix);
        assert_eq!(hit.owner.as_deref(), Some(owner));
    }
    assert_eq!(outcomes[4].1, RuleOutcome::NotAvailable {
        rule: "tessera-slop".into(), engine: Engine::Tessera,
        why: "Tessera's slop records are not defined yet".into(),
    });
    assert_eq!(filtered.iter().map(|(id, _)| id.as_str()).collect::<Vec<_>>(), ids);
    for index in [0, 1, 3] {
        assert_eq!(filtered[index].1, RuleOutcome::Findings(vec![]));
    }
    assert_eq!(filtered[2].1, outcomes[2].1);
    assert_eq!(filtered[4].1, outcomes[4].1);
}

#[test]
fn clippy_workspace_member_finding_is_relative_to_rule_folder() {
    assert_eq!(availability(Engine::Clippy), Availability::Available);
    let folder = TempFolder::new("member");
    let member = folder.0.join("member");
    fs::create_dir(&member).unwrap();
    fs::create_dir(member.join("src")).unwrap();
    fs::write(folder.0.join("Cargo.toml"), "[workspace]\nmembers = [\"member\"]\nresolver = \"2\"\n").unwrap();
    fs::write(member.join("Cargo.toml"), "[package]\nname = \"member_zit\"\nversion = \"0.1.0\"\nedition = \"2021\"\n").unwrap();
    fs::write(member.join("src/lib.rs"), "pub fn compare(value: bool) -> bool {\n    value == true\n}\n").unwrap();
    fs::write(member.join("temperare.toml"), "version = 1\n[[rule]]\nid = \"member-bool\"\nengine = \"clippy\"\ncheck = \"clippy::bool_comparison\"\nfix = \"Simplify.\"\n").unwrap();
    let file = RuleFile::read(&member.join("temperare.toml")).unwrap();
    let outcomes = outcomes_of(&file, &folder.0, &[]);
    drop(folder);
    assert_eq!(outcomes.len(), 1);
    assert_eq!(outcomes[0].0, "member-bool");
    let RuleOutcome::Findings(findings) = &outcomes[0].1 else { panic!("{:?}", outcomes[0].1) };
    assert_eq!(findings.len(), 1, "{findings:?}");
    assert_eq!(findings[0].path, RelPath::new("src/lib.rs"));
    assert_eq!(findings[0].line(), Some(2));
    assert!(findings[0].what.contains("true"), "{:?}", findings[0]);
}

#[test]
fn clippy_warnings_cover_all_targets_deduplicate_and_honor_allows() {
    assert_eq!(availability(Engine::Clippy), Availability::Available);
    let folder = TempFolder::new("warnings");
    fs::create_dir(folder.0.join("src")).unwrap();
    fs::write(folder.0.join("Cargo.toml"), "[package]\nname = \"warnings_zit\"\nversion = \"0.1.0\"\nedition = \"2021\"\n[workspace]\n").unwrap();
    fs::write(folder.0.join("src/lib.rs"), concat!(
        "pub fn compare(value: bool) -> bool {\n",
        "    value == true\n",
        "}\n",
        "pub fn big() -> Result<(), [u8; 256]> { Ok(()) }\n",
        "pub mod things { pub mod things { pub fn f() {} } }\n",
        "#[cfg(test)]\n",
        "mod tests {\n",
        "    #[test]\n",
        "    fn check() {\n",
        "        let unused = 1;\n",
        "    }\n",
        "}\n",
    )).unwrap();
    fs::write(folder.0.join("temperare.toml"), r#"version = 1
[[rule]]
id = "allowed"
engine = "clippy"
check = "warnings"
severity = "error"
fix = "Fix warning."
params = { allow = ["clippy::result_large_err", "clippy::module_inception"] }
[[rule]]
id = "control"
engine = "clippy"
check = "warnings"
severity = "error"
fix = "Fix warning."
"#).unwrap();
    let file = RuleFile::read(&folder.0.join("temperare.toml")).unwrap();
    let outcomes = outcomes_of(&file, &folder.0, &[]);
    assert_eq!(outcomes.len(), 2);
    assert_eq!(outcomes[0].0, "allowed");
    assert_eq!(outcomes[1].0, "control");
    let RuleOutcome::Findings(allowed) = &outcomes[0].1 else { panic!("allowed: {:?}", outcomes[0].1) };
    let RuleOutcome::Findings(control) = &outcomes[1].1 else { panic!("control: {:?}", outcomes[1].1) };
    assert!(allowed.iter().any(|f| f.path == RelPath::new("src/lib.rs") && f.line() == Some(10)
        && f.what.starts_with("unused_variables: ") && f.severity == Severity::Error), "{allowed:?}");
    assert_eq!(allowed.iter().filter(|f| f.path == RelPath::new("src/lib.rs") && f.line() == Some(2)
        && f.what.starts_with("clippy::bool_comparison: ")).count(), 1, "{allowed:?}");
    for (code, line) in [("clippy::result_large_err", 4), ("clippy::module_inception", 5)] {
        assert!(!allowed.iter().any(|f| f.what.starts_with(code)), "{allowed:?}");
        assert!(control.iter().any(|f| f.path == RelPath::new("src/lib.rs") && f.line() == Some(line)
            && f.what.starts_with(&format!("{code}: "))), "{control:?}");
    }
}

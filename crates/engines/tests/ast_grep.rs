use std::fs;

use engines::{availability, ast_grep::{fill_params, parse_ast_grep, AST_GREP}, entry_cmd, run_rules_versioned_at, Availability, TempFile};
use rules::{Engine, RelPath, RuleFile, RuleOutcome};

#[test]
fn ast_grep_runs_a_list_parameterized_rule_and_removes_the_temp_file() {
    assert_eq!(availability(Engine::AstGrep), Availability::Available);
    let folder = std::env::temp_dir().join(format!("engines-zit-{}-ast-grep", std::process::id()));
    fs::create_dir(&folder).unwrap();
    let folder = fs::canonicalize(folder).unwrap();
    fs::write(folder.join("example.js"), "console.log(foo);\nconsole.log(baz);\n").unwrap();
    let yaml = "id: selected-names\nlanguage: JavaScript\nmessage: matched name\nseverity: warning\nrule:\n  kind: identifier\n  regex: '^(?:{{names}})$'\n";
    let rule = fill_params(yaml, &serde_json::json!({"names": ["foo", "bar"]})).unwrap();
    let temp = TempFile::write("ast-grep", "yml", &rule).unwrap();
    let rule_path = temp.path.clone();
    let result = procio::run(&entry_cmd(&AST_GREP, temp.path.as_os_str(), &[], None, &folder))
        .map(|out| parse_ast_grep(&out, &folder, &folder));
    drop(temp);
    let rule_removed = !rule_path.exists();
    fs::remove_dir_all(&folder).unwrap();
    assert!(rule_removed, "temporary rule remains at {}", rule_path.display());
    let hits = result.unwrap().unwrap();
    assert_eq!(hits.len(), 1, "{hits:?}");
    assert_eq!(hits[0].path, RelPath::new("example.js"));
    assert_eq!(hits[0].line, Some(1));
    assert_eq!(hits[0].col, Some(13));
    assert!(hits[0].what.contains("matched name"), "{:?}", hits[0]);
}

#[test]
fn run_rules_rejects_ast_grep_yaml_outside_the_rule_folder() {
    let root = std::env::temp_dir().join(format!("engines-zit-{}-ast-grep-outside", std::process::id()));
    fs::create_dir(&root).unwrap();
    let root = fs::canonicalize(root).unwrap();
    let folder = root.join("rules");
    fs::create_dir(&folder).unwrap();
    fs::write(root.join("outside.yml"), "id: names\nlanguage: JavaScript\nmessage: matched name\nseverity: warning\nrule:\n  kind: identifier\n").unwrap();
    fs::write(folder.join("temperare.toml"), "version = 1\n[[rule]]\nid = \"outside\"\nengine = \"ast-grep\"\ncheck = \"../outside.yml\"\nfix = \"f\"\n").unwrap();
    let file = RuleFile::read(&folder.join("temperare.toml")).unwrap();
    let outcomes = run_rules_versioned_at(&file, &root, &[], None).runs.into_iter().map(|run| (run.rule, run.outcome)).collect::<Vec<_>>();
    fs::remove_dir_all(&root).unwrap();
    assert_eq!(outcomes, vec![("outside".to_string(), RuleOutcome::Failed {
        rule: "outside".to_string(),
        engine: Engine::AstGrep,
        code: None,
        stderr: "ast-grep rule ../outside.yml is outside the rule file's folder".to_string(),
    })]);
}

const EMBEDDED_YAML: &str = "id: selected-names\nlanguage: JavaScript\nmessage: embedded matched name\nseverity: warning\nrule:\n  kind: identifier\n  regex: '^(?:{{names}})$'\n";
const BROKEN_YAML: &str = "id: broken\nlanguage: NotALanguage\nrule:\n  kind: identifier\n";
const REPOSITORY_YAML: &str = "id: repository-names\nlanguage: JavaScript\nmessage: repository matched name\nseverity: warning\nrule:\n  kind: identifier\n  regex: '^(?:{{names}})-never$'\n";
const EMBEDDED_RULE: &str = "version = 1\n[[rule]]\nid = \"embedded\"\nengine = \"ast-grep\"\ncheck = \"baseline.yml\"\nfix = \"Rename the identifier.\"\nparams = { names = [\"foo\"] }\n";
const UNPARAMETERIZED_RULE: &str = "version = 1\n[[rule]]\nid = \"embedded\"\nengine = \"ast-grep\"\ncheck = \"baseline.yml\"\nfix = \"Rename the identifier.\"\n";

fn embedded_outcomes(tag: &str, rule: &str, yaml: &'static str, repository_yaml: Option<&str>) -> Vec<RuleOutcome> {
    let folder = std::env::temp_dir().join(format!("engines-zit-{}-{tag}", std::process::id()));
    fs::create_dir(&folder).unwrap();
    let folder = fs::canonicalize(folder).unwrap();
    fs::write(folder.join("example.js"), "console.log(foo);\n").unwrap();
    if let Some(text) = repository_yaml {
        fs::write(folder.join("baseline.yml"), text).unwrap();
    }
    let mut file = RuleFile::parse(rule, &folder).unwrap();
    file.rules[0].embedded_text = Some(yaml);
    let outcomes = run_rules_versioned_at(&file, &folder, &[], None).runs.into_iter().map(|run| run.outcome).collect();
    fs::remove_dir_all(&folder).unwrap();
    outcomes
}

fn assert_embedded_finding(outcomes: Vec<RuleOutcome>) {
    assert_eq!(outcomes.len(), 1, "{outcomes:?}");
    let RuleOutcome::Findings(findings) = &outcomes[0] else {
        panic!("expected findings, got {:?}", outcomes[0]);
    };
    assert_eq!(findings.len(), 1, "{findings:?}");
    assert_eq!(findings[0].path, RelPath::new("example.js"));
    assert_eq!(findings[0].line, 1);
    assert_eq!(findings[0].what, "embedded matched name");
}

#[test]
fn embedded_ast_grep_finds_violation_with_filled_params_and_no_repository_yaml() {
    assert_eq!(availability(Engine::AstGrep), Availability::Available);
    assert_embedded_finding(embedded_outcomes("embedded-params", EMBEDDED_RULE, EMBEDDED_YAML, None));
}

#[test]
fn embedded_ast_grep_yaml_takes_precedence_over_repository_yaml() {
    assert_eq!(availability(Engine::AstGrep), Availability::Available);
    assert_embedded_finding(embedded_outcomes("embedded-precedence", EMBEDDED_RULE, EMBEDDED_YAML, Some(REPOSITORY_YAML)));
}

#[test]
fn rejected_embedded_ast_grep_yaml_reports_engine_failure_with_exit_code() {
    assert_eq!(availability(Engine::AstGrep), Availability::Available);
    let outcomes = embedded_outcomes("embedded-invalid", UNPARAMETERIZED_RULE, BROKEN_YAML, None);
    assert_eq!(outcomes.len(), 1, "{outcomes:?}");
    let RuleOutcome::Failed { rule, engine, code, stderr } = &outcomes[0] else {
        panic!("expected failure, got {:?}", outcomes[0]);
    };
    assert_eq!(rule, "embedded");
    assert_eq!(*engine, Engine::AstGrep);
    assert!(code.is_some_and(|code| code != 0), "exit code: {code:?}");
    assert!(stderr.contains("NotALanguage"), "stderr: {stderr}");
}

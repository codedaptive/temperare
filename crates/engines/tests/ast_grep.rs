use std::fs;

use engines::{availability, ast_grep::{ast_grep_cmd, fill_params, parse_ast_grep, TempRule}, run_rules, Availability};
use rules::{Engine, RelPath, RuleFile, RuleOutcome};

#[test]
fn ast_grep_runs_a_list_parameterized_rule_and_removes_the_temp_rule() {
    assert_eq!(availability(Engine::AstGrep), Availability::Available);
    let folder = std::env::temp_dir().join(format!("engines-zit-{}-ast-grep", std::process::id()));
    fs::create_dir(&folder).unwrap();
    let folder = fs::canonicalize(folder).unwrap();
    fs::write(folder.join("example.js"), "console.log(foo);\nconsole.log(baz);\n").unwrap();
    let yaml = "id: selected-names\nlanguage: JavaScript\nmessage: matched name\nseverity: warning\nrule:\n  kind: identifier\n  regex: '^(?:{{names}})$'\n";
    let rule = fill_params(yaml, &serde_json::json!({"names": ["foo", "bar"]})).unwrap();
    let temp = TempRule::write(&rule).unwrap();
    let rule_path = temp.path.clone();
    let result = procio::run(&ast_grep_cmd(&temp.path, &folder))
        .map(|out| parse_ast_grep(&out, &folder, &folder));
    drop(temp);
    let rule_removed = !rule_path.exists();
    fs::remove_dir_all(&folder).unwrap();
    assert!(rule_removed, "temporary rule remains at {}", rule_path.display());
    let hits = result.unwrap().unwrap();
    assert_eq!(hits.len(), 1, "{hits:?}");
    assert_eq!(hits[0].path, RelPath::new("example.js"));
    assert_eq!(hits[0].line, 1);
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
    let outcomes = run_rules(&file, &root, &[]);
    fs::remove_dir_all(&root).unwrap();
    assert_eq!(outcomes, vec![("outside".to_string(), RuleOutcome::Failed {
        rule: "outside".to_string(),
        engine: Engine::AstGrep,
        code: None,
        stderr: "ast-grep rule ../outside.yml is outside the rule file's folder".to_string(),
    })]);
}

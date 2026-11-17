use std::fs;

use engines::{availability, ast_grep::{ast_grep_cmd, fill_params, parse_ast_grep, TempRule}, Availability};
use rules::{Engine, RelPath};

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

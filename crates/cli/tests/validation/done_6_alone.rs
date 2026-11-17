//! Done 6: alone. "All of the above run with no codeteam variable set." Every case in this suite runs `tm` in the
//! environment `env.rs` builds from nothing; this case goes further and gives `tm` only PATH and HOME, in a plain git
//! repository with no Tessera rule (mission must match 1: useful alone).

use serde_json::Value;

use crate::env::clean_env;
use crate::repo::Repo;
use crate::run_clean::run_clean;

const RULES: &str = r#"version = 1

[[rule]]
id = "no-none-compare"
engine = "ruff"
check = "E711"
severity = "error"
fix = "Compare with `is None`."
"#;

#[test]
fn tm_checks_a_plain_git_repository_with_only_path_and_home() {
    let repo = Repo::new();
    repo.write("temperare.toml", RULES);
    repo.write("app.py", "def f(x):\n    return x is None\n");
    repo.commit("plain");
    let keep: Vec<(String, String)> =
        clean_env(None).into_iter().filter(|(k, _)| k == "PATH" || k == "HOME").collect();
    let out = run_clean(env!("CARGO_BIN_EXE_tm"), &["check", "--json"], repo.path(), keep);
    let json: Value = serde_json::from_slice(&out.stdout).expect("tm --json prints JSON");
    assert_eq!(out.code, Some(0), "clean: {json:#}\n{}", String::from_utf8_lossy(&out.stderr));
    assert_eq!(json["findings"].as_array().map(Vec::len), Some(0), "no findings: {json:#}");
}

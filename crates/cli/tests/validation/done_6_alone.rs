//! Done 6: alone. "All of the above run with no codeteam variable set." Every case in this suite runs `cctao` with
//! only the variables it sets (`env.rs` for the acceptance cases, `install_temp_home.rs` its own); this case goes further
//! and gives `cctao` only PATH and HOME (HOME a folder the test made), in a plain git repository with no Tessera
//! rule (mission must match 1: useful alone).

use serde_json::Value;

use crate::check::Check;
use crate::env::cctao_env;
use crate::repo::Repo;
use crate::run_clean::run_clean;
use crate::temp_home::TempHome;

const RULES: &str = r#"version = 1

[[rule]]
id = "no-none-compare"
engine = "ruff"
check = "E711"
severity = "error"
fix = "Compare with `is None`."
"#;

#[test]
fn cctao_checks_a_plain_git_repository_with_only_path_and_home() {
    let mut check = Check::new(
        "cctao_checks_a_plain_git_repository_with_only_path_and_home",
        "a plain git repository with rule no-none-compare (ruff, E711) and a clean app.py; cctao gets only PATH and HOME",
    );
    let repo = Repo::new();
    repo.write("tao.toml", RULES);
    repo.write("app.py", "def f(x):\n    return x is None\n");
    repo.commit("plain");
    let home = TempHome::new();
    let keep: Vec<(String, String)> =
        cctao_env(None, home.path()).into_iter().filter(|(k, _)| k == "PATH" || k == "HOME").collect();
    let args = ["check", "--json"];
    let out = run_clean(env!("CARGO_BIN_EXE_cctao"), &args, repo.path(), keep);
    let json: Value = serde_json::from_slice(&out.stdout).expect("cctao --json prints JSON");
    check.saw_cctao(&args, out.code.expect("cctao ended by a signal"), &json);
    assert_eq!(out.code, Some(0), "clean: {json:#}\n{}", String::from_utf8_lossy(&out.stderr));
    assert_eq!(json["findings"].as_array().map(Vec::len), Some(0), "no findings: {json:#}");
}

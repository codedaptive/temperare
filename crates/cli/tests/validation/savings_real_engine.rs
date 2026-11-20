//! Savings from a real ruff run in a throwaway repository and HOME.

use crate::env::tm_env;
use crate::on_tm_path::on_tm_path;
use crate::repo::Repo;
use crate::run_clean::run_clean;
use crate::temp_home::TempHome;

const RULES: &str = r#"version = 1
[[rule]]
id = "unused-import"
engine = "ruff"
check = "F401"
severity = "error"
fix = "Remove the unused import."
"#;

#[test]
fn real_ruff_text_baseline_matches_json_or_reports_unavailable() {
    let repo = Repo::new();
    let home = TempHome::new();
    repo.write("temperare.toml", RULES);
    repo.write("a.py", "import os\n");
    repo.commit("unused import");
    let text = run_clean(env!("CARGO_BIN_EXE_temperare"), &["check"], repo.path(), tm_env(None, home.path()));
    let stdout = String::from_utf8(text.stdout).unwrap();
    if !on_tm_path("ruff") {
        assert_eq!(text.code, Some(3), "{stdout}");
        assert!(!stdout.contains("[temperare] tokens saved"), "{stdout}");
        assert!(stdout.contains("not available: unused-import (ruff):"), "{stdout}");
        return;
    }
    assert_eq!(text.code, Some(1), "{stdout}\n{}", String::from_utf8_lossy(&text.stderr));
    let line = stdout.lines().next().unwrap();
    assert!(line.starts_with("[temperare] tokens saved ≈ "), "{line}");
    assert!(line.contains("1 engine run(s)"), "{line}");
    let baseline: u64 = line.split_once(" tok (estimate)").unwrap().0.rsplit_once('≈').unwrap().1.trim().replace(',', "").parse().unwrap();
    assert!(baseline > 0, "{line}");
    let json = run_clean(env!("CARGO_BIN_EXE_temperare"), &["check", "--json"], repo.path(), tm_env(None, home.path()));
    assert_eq!(json.code, Some(1), "{}", String::from_utf8_lossy(&json.stderr));
    let value: serde_json::Value = serde_json::from_slice(&json.stdout).unwrap();
    assert_eq!(value["saved"]["engine_runs"], 1);
    assert_eq!(value["saved"]["baseline_tokens"], baseline);
}

//! Acceptance check, the shared run: two ruff rules that do not overlap, with no config and no params, are
//! checked by one ruff run. It pins exit 1, one engine run on the printed line, one `engines` entry carrying ruff's
//! own version word, the saved output counted once (on the first rule, 0 on the second), and each rule's own finding
//! at its place. When ruff is not on the PATH `cctao` gets (`on_cctao_path`), the case pins `cctao`'s answer
//! (exit 3, both rules not available, no findings, an empty `engines` list) and is NOT RUN with `cctao`'s reason,
//! never PASS.

use serde_json::{json, Value};

use crate::check::Check;
use crate::env::cctao_env;
use crate::on_cctao_path::on_cctao_path;
use crate::repo::Repo;
use crate::run_clean::run_clean;
use crate::temp_home::TempHome;

const RULES: &str = r#"version = 1
[[rule]]
id = "no-none-compare"
engine = "ruff"
check = "E711"
fix = "Compare with None using `is`."

[[rule]]
id = "unused-import"
engine = "ruff"
check = "F401"
fix = "Remove the unused import."
"#;

// Line 1 imports os unused (F401); line 3 compares with `== None` (E711).
const SOURCE: &str = "import os\nx = None\nif x == None:\n    pass\n";

#[test]
fn two_ruff_rules_share_one_engine_run() {
    let mut check = Check::new(
        "two_ruff_rules_share_one_engine_run",
        "rules no-none-compare (ruff, E711) and unused-import (ruff, F401), no config or params; a.py line 1 imports \
         os unused, line 3 compares with == None",
    );
    let repo = Repo::new();
    let home = TempHome::new();
    repo.write("tao.toml", RULES);
    repo.write("a.py", SOURCE);
    repo.commit("two ruff rules");
    let args = ["check", "--json"];
    let output = run_clean(env!("CARGO_BIN_EXE_cctao"), &args, repo.path(), cctao_env(None, home.path()));
    let code = output.code.expect("run_clean returns only runs that exited");
    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    let value: Value =
        serde_json::from_str(&stdout).unwrap_or_else(|e| panic!("stdout is not JSON ({e}): {stdout}\n{stderr}"));
    check.saw_cctao(&args, code, &value);
    if !on_cctao_path("ruff") {
        check.not_run(absent_ruff_reason(code, &stdout, &value));
        return;
    }
    assert_eq!(code, 1, "{stdout}\n{stderr}");
    // The version is machine-specific, so it is read from the same ruff cctao runs, for this expected value only; for
    // ruff's `ruff X.Y.Z` output, word nth(1) is the same word engines' `version_word` takes.
    let probe = run_clean("ruff", &["--version"], repo.path(), cctao_env(None, home.path()));
    assert_eq!(probe.code, Some(0));
    let probe_stdout = String::from_utf8(probe.stdout).unwrap();
    let version = probe_stdout.split_whitespace().nth(1).expect("ruff version");
    assert!(stdout.contains(&format!(r#""engines":[{{"engine":"ruff","version":"{version}"}}]"#)), "{stdout}");
    assert_eq!(value["engines"].as_array().map(Vec::len), Some(1), "{stdout}");
    // Measured on the printed line itself: one `"engine_runs":` whose value is the number 1.
    let pieces: Vec<&str> = stdout.split("\"engine_runs\":").collect();
    assert_eq!(pieces.len(), 2, "exactly one engine_runs: {stdout}");
    assert!(pieces[1].starts_with("1,") || pieces[1].starts_with("1}"), "engine_runs is not 1: {stdout}");

    assert!(empty_or_absent(&value, "not_available"), "{stdout}");
    assert!(empty_or_absent(&value, "failed"), "{stdout}");
    let counted = value["saved"]["counted"].as_array().unwrap_or_else(|| panic!("no saved.counted: {stdout}"));
    assert_eq!(counted.len(), 2, "{stdout}");
    assert_eq!(counted[0]["rule"], "no-none-compare", "{stdout}");
    assert_eq!(counted[0]["engine"], "ruff", "{stdout}");
    assert!(counted[0]["chars"].as_u64().is_some_and(|n| n > 0), "the run's output is counted once: {stdout}");
    assert_eq!(counted[1], json!({ "rule": "unused-import", "engine": "ruff", "chars": 0 }), "{stdout}");

    let findings = value["findings"].as_array().unwrap_or_else(|| panic!("no findings: {stdout}"));
    assert_eq!(findings.len(), 2, "{stdout}");
    assert_eq!(place(findings, "no-none-compare"), ("a.py", 3), "{stdout}");
    assert_eq!(place(findings, "unused-import"), ("a.py", 1), "{stdout}");
}

/// With ruff absent: asserts `cctao`'s answer (exit 3, both rules not available, no findings, an empty `engines`
/// list, since an engine cctao did not find is left out) and returns NOT RUN's reason, `cctao`'s own `why` for no-none-compare.
fn absent_ruff_reason(code: i32, stdout: &str, value: &Value) -> String {
    assert_eq!(code, 3, "{stdout}");
    let list = value["not_available"].as_array().unwrap_or_else(|| panic!("no not_available list: {stdout}"));
    let mut missing: Vec<(&str, &str)> =
        list.iter().map(|n| (n["rule"].as_str().unwrap_or(""), n["engine"].as_str().unwrap_or(""))).collect();
    missing.sort();
    assert_eq!(missing, vec![("no-none-compare", "ruff"), ("unused-import", "ruff")], "{stdout}");
    assert!(empty_or_absent(value, "findings"), "{stdout}");
    assert!(stdout.contains(r#""engines":[]"#), "{stdout}");
    assert_eq!(value["engines"], json!([]), "{stdout}");
    let why = list
        .iter()
        .find(|n| n["rule"] == "no-none-compare")
        .and_then(|n| n["why"].as_str())
        .unwrap_or_else(|| panic!("no-none-compare is not available without a reason: {stdout}"));
    format!("cctao: {why}")
}

/// True when `key` is missing, null or an empty list.
fn empty_or_absent(value: &Value, key: &str) -> bool {
    value.get(key).is_none_or(|v| v.is_null() || v.as_array().is_some_and(Vec::is_empty))
}

/// The path and line of the one finding for `rule`.
fn place<'a>(findings: &'a [Value], rule: &str) -> (&'a str, u64) {
    let mine: Vec<&Value> = findings.iter().filter(|f| f["rule"] == rule).collect();
    assert_eq!(mine.len(), 1, "one finding for {rule}: {findings:?}");
    (mine[0]["path"].as_str().unwrap_or(""), mine[0]["line"].as_u64().unwrap_or(0))
}

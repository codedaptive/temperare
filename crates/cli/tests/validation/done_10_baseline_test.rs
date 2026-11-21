//! Done 10's acceptance check, the baseline: the built `temperare` runs `baseline test --json` over the ready sets tm
//! tests without `--set` (whichever sets `rules::baseline_sets` marks ready), in a clean environment with HOME
//! and cwd folders the test made. It never pins the sets' contents: the answer's `exit` is the process's,
//! `counts.rules` is the length of `rules`, and no rule is broken. Exit 0 is PASS with every rule holding; exit 3
//! (an engine absent here, or a case not applicable, such as one that gives no value for a required param) is NOT
//! RUN, naming each not-available or not-applicable rule and its why; any other exit fails.

use serde_json::Value;

use crate::check::Check;
use crate::env::tm_env;
use crate::run_clean::run_clean;
use crate::temp_home::TempHome;

#[test]
fn the_shipped_baseline_sets_hold() {
    let mut check = Check::new(
        "the_shipped_baseline_sets_hold",
        "every rule of the ready baseline sets (no --set), each on its own failing and passing case; no repository",
    );
    let home = TempHome::new();
    let cwd = TempHome::new();
    let args = ["baseline", "test", "--json"];
    let output = run_clean(env!("CARGO_BIN_EXE_temperare"), &args, cwd.path(), tm_env(None, home.path()));
    let code = output.code.expect("run_clean returns only runs that exited");
    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    let value: Value =
        serde_json::from_str(&stdout).unwrap_or_else(|e| panic!("stdout is not JSON ({e}): {stdout}\n{stderr}"));
    check.saw_tm(&args, code, &value);
    assert_eq!(value["exit"].as_i64(), Some(i64::from(code)), "{stdout}\n{stderr}");
    let rules = value["rules"].as_array().unwrap_or_else(|| panic!("no rules list: {stdout}"));
    let counted = value["counts"]["rules"].as_u64().and_then(|n| usize::try_from(n).ok());
    assert_eq!(counted, Some(rules.len()), "{stdout}");
    assert_eq!(value["counts"]["broken"].as_u64(), Some(0), "{stdout}\n{stderr}");
    let broken: Vec<&Value> = rules.iter().filter(|r| r["result"] == "broken").collect();
    assert!(broken.is_empty(), "broken rules: {broken:?}");
    match code {
        0 => assert!(rules.iter().all(|r| r["result"] == "holds"), "exit 0 with a rule that does not hold: {stdout}"),
        3 => check.not_run(not_run_reason(rules, &stdout)),
        _ => panic!("baseline test exited {code}: {stdout}\n{stderr}"),
    }
}

/// NOT RUN's reason: `tm: ` and, for each not-available or not-applicable rule in order,
/// `<set> <rule> (<engine>): <why>`, the why of its first case with the rule's result; asserts there is at least one.
fn not_run_reason(rules: &[Value], stdout: &str) -> String {
    let named: Vec<String> = rules
        .iter()
        .filter(|r| r["result"] == "not_available" || r["result"] == "not_applicable")
        .map(|r| {
            let why = [&r["failing"], &r["passing"]]
                .into_iter()
                .find(|case| case["result"] == r["result"])
                .and_then(|case| case["why"].as_str())
                .unwrap_or_else(|| panic!("a rule not run without a why: {r}"));
            format!("{} {} ({}): {why}", str_of(r, "set"), str_of(r, "rule"), str_of(r, "engine"))
        })
        .collect();
    assert!(!named.is_empty(), "exit 3 with no rule not available or not applicable: {stdout}");
    format!("tm: {}", named.join("; "))
}

/// The string at `key`, or empty.
fn str_of<'a>(rule: &'a Value, key: &str) -> &'a str {
    rule[key].as_str().unwrap_or("")
}

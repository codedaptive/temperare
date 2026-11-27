//! `cctao baseline test` on the built `cctao`, in a clean environment with HOME and cwd folders the test made (no
//! repository is needed). The ready sets' contents change as rules adds to them, so their runs are held only to
//! facts that hold for any contents: the JSON form's one line, its `exit` and its rule count, and the text form's
//! last line. An unknown `--set` is pinned by its `exit`, its empty stdout and the first line of its stderr.

use serde_json::Value;

use crate::env::cctao_env;
use crate::run_clean::run_clean;
use crate::temp_home::TempHome;

/// `cctao <args>` with HOME and cwd folders of its own: (exit, stdout, stderr).
fn cctao(args: &[&str]) -> (i32, String, String) {
    let home = TempHome::new();
    let cwd = TempHome::new();
    let out = run_clean(env!("CARGO_BIN_EXE_cctao"), args, cwd.path(), cctao_env(None, home.path()));
    let code = out.code.expect("run_clean returns only runs that exited");
    (code, String::from_utf8_lossy(&out.stdout).into_owned(), String::from_utf8_lossy(&out.stderr).into_owned())
}

#[test]
fn baseline_test_on_the_built_binary() {
    let (code, stdout, stderr) = cctao(&["baseline", "test", "--set", "nosuch@1"]);
    assert_eq!(code, 2, "unknown set: stdout {stdout:?}, stderr {stderr:?}");
    assert_eq!(stdout, "", "unknown set: stderr {stderr:?}");
    assert_eq!(stderr.lines().next(), Some("cctao: unknown baseline set \"nosuch\""), "unknown set: {stderr:?}");

    let (code, stdout, stderr) = cctao(&["baseline", "test", "--json"]);
    assert!(stdout.ends_with('\n'), "--json: {stdout:?}\n{stderr}");
    assert_eq!(stdout.lines().count(), 1, "--json prints one line: {stdout:?}");
    let value: Value =
        serde_json::from_str(&stdout).unwrap_or_else(|e| panic!("--json is not JSON ({e}): {stdout}\n{stderr}"));
    assert_eq!(value["exit"].as_i64(), Some(i64::from(code)), "--json: {stdout}\n{stderr}");
    let rules = value["rules"].as_array().unwrap_or_else(|| panic!("--json: no rules list: {stdout}")).len();
    let counted = value["counts"]["rules"].as_u64().and_then(|n| usize::try_from(n).ok());
    assert_eq!(counted, Some(rules), "--json: counts.rules against rules: {stdout}");

    let (_, stdout, stderr) = cctao(&["baseline", "test"]);
    let last = stdout.lines().last().unwrap_or("");
    assert!(last.starts_with("next: "), "text's last line {last:?}: {stdout}\n{stderr}");
}

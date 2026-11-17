//! Done 2: new violations only. "With one violation at the base commit and one added after it, only the added one is
//! reported." A violation in a new, untracked file is new too (S-14, S-19).

use crate::repo::Repo;
use crate::tm::tm;

const RULES: &str = r#"version = 1

[[rule]]
id = "no-none-compare"
engine = "ruff"
check = "E711"
severity = "error"
fix = "Compare with `is None`."
"#;

const OLD: &str = "def old(x):\n    return x == None\n";

fn repo_with_base() -> (Repo, String) {
    let repo = Repo::new();
    repo.write("temperare.toml", RULES);
    repo.write("app.py", OLD);
    let base = repo.commit("base: one violation");
    (repo, base)
}

#[test]
fn only_the_violation_added_after_the_base_is_reported() {
    let (repo, base) = repo_with_base();
    repo.write("app.py", &format!("{OLD}\n\ndef new(y):\n    return y == None\n"));
    repo.commit("adds a second violation");

    let all = tm(&repo, &["check", "--json"]);
    let both = vec![(String::from("app.py"), 2), (String::from("app.py"), 6)];
    assert_eq!(all.places(), both, "both, with no base: {:#}", all.json);

    let a = tm(&repo, &["check", "--json", "--base", &base]);
    assert_eq!(a.code, 1, "the added violation refuses: {:#}\n{}", a.json, a.stderr);
    assert_eq!(a.places(), vec![(String::from("app.py"), 6)], "only the added one: {:#}", a.json);
    assert_eq!(a.json["base"], base.as_str(), "the report names its base");
}

#[test]
fn a_violation_in_a_new_untracked_file_is_new() {
    let (repo, base) = repo_with_base();
    repo.write("fresh.py", "def fresh(z):\n    return z == None\n");
    let a = tm(&repo, &["check", "--json", "--base", &base]);
    assert_eq!(a.places(), vec![(String::from("fresh.py"), 2)], "the untracked file's violation only: {:#}", a.json);
}

#[test]
fn a_base_that_does_not_resolve_is_a_usage_error() {
    let (repo, _) = repo_with_base();
    let a = tm(&repo, &["check", "--json", "--base", "no-such-commit"]);
    assert_eq!(a.code, 2, "never an empty report (SPEC-report WO-report-3, S-15): {:#}", a.json);
}

//! Done 9 (actionlint) (SPEC-engines-common Done 9): a rule naming the engine reports a known violation, in a file the
//! test made, with its rule, its place and its fix text, and the report lists the engine's version; with the engine
//! off the PATH, the same rule reports "not available" (exit 3). One rule, `engine = "actionlint"`,
//! `check = "expression"`; the workflows sit directly in .github/workflows, where the adapter looks: ci.yml reads the
//! undefined context `unknown` on line 7, ok.yml echoes a plain word. tm runs with PATH the folder that gives it
//! actionlint (`tm_path_for`) then /usr/bin:/bin. Only when actionlint is not on the PATH `temperare` gets
//! (`on_tm_path`, which runs nothing) is the reported case NOT RUN, with `temperare`'s reason, never PASS;
//! `temperare` must then report the rule not available.

use serde_json::json;

use crate::absent_engine_reason::absent_engine_reason;
use crate::check::Check;
use crate::not_available_pairs::not_available_pairs;
use crate::probed_version::probed_version;
use crate::repo::Repo;
use crate::tm::tm_with_path;
use crate::tm_path_for::tm_path_for;

const RULE: &str = "workflow-expression";
const FIX: &str = "Read a context the workflow defines.";

const RULES: &str = r#"version = 1

[[rule]]
id = "workflow-expression"
engine = "actionlint"
check = "expression"
severity = "error"
fix = "Read a context the workflow defines."
"#;

// Line 7 of EXAMPLE reads `unknown.x`, a context no workflow defines; CLEAN reads none.
const EXAMPLE: &str = "name: CI\non: push\njobs:\n  build:\n    runs-on: ubuntu-latest\n    steps:\n      \
                       - run: echo \"${{ unknown.x }}\"\n";
const CLEAN: &str = "name: OK\non: push\njobs:\n  build:\n    runs-on: ubuntu-latest\n    steps:\n      \
                     - run: echo hello\n";

/// A repository holding the rule file and the two workflows, committed.
fn workflow_repo() -> Repo {
    let repo = Repo::new();
    repo.write("temperare.toml", RULES);
    repo.write(".github/workflows/ci.yml", EXAMPLE);
    repo.write(".github/workflows/ok.yml", CLEAN);
    repo.commit("workflows");
    repo
}

#[test]
fn an_actionlint_error_is_reported_with_its_rule_place_fix_and_version() {
    let mut check = Check::new(
        "an_actionlint_error_is_reported_with_its_rule_place_fix_and_version",
        "rule workflow-expression (actionlint, expression); .github/workflows/ci.yml line 7 reads `${{ unknown.x }}`, \
         ok.yml reads no context; PATH is actionlint's folder then /usr/bin:/bin",
    );
    let repo = workflow_repo();
    let args = ["check", "--json"];
    let a = tm_with_path(&repo, &args, Some(&tm_path_for(&["actionlint"])));
    check.saw_tm(&args, a.code, &a.json);
    if let Some(why) = absent_engine_reason(&a, &["actionlint"], RULE) {
        check.not_run(why);
        return;
    }
    assert_eq!(a.code, 1, "a quality error is the refusal (S-15): {:#}\n{}", a.json, a.stderr);
    let f = a.findings();
    assert_eq!(f.len(), 1, "one finding, in ci.yml; ok.yml passes: {:#}", a.json);
    assert_eq!(f[0]["rule"], RULE);
    assert_eq!(f[0]["engine"], "actionlint");
    assert_eq!(f[0]["path"], ".github/workflows/ci.yml");
    assert_eq!(f[0]["line"], 7);
    assert_eq!(f[0]["fix"], FIX);
    let version = probed_version("actionlint", &["-version"], repo.path());
    assert_eq!(a.json["engines"], json!([{ "engine": "actionlint", "version": version }]), "{:#}", a.json);
}

#[test]
fn actionlint_off_path_is_not_available() {
    let mut check = Check::new(
        "actionlint_off_path_is_not_available",
        "rule workflow-expression (actionlint, expression) over the same files; tm runs with PATH /usr/bin:/bin, \
         which holds no actionlint",
    );
    let repo = workflow_repo();
    let args = ["check", "--json"];
    let a = tm_with_path(&repo, &args, Some("/usr/bin:/bin"));
    check.saw_tm(&args, a.code, &a.json);
    assert_eq!(a.code, 3, "not available has its own code: {:#}\n{}", a.json, a.stderr);
    assert!(a.findings().is_empty(), "nothing is reported as found: {:#}", a.json);
    assert_eq!(not_available_pairs(&a), vec![(RULE.to_string(), "actionlint".to_string())], "{:#}", a.json);
}

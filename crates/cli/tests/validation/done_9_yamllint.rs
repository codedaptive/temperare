//! Done 9 (yamllint) (SPEC-engines-common Done 9): a rule naming the engine reports a known violation, in a file the
//! test made, with its rule, its place and its fix text, and the report lists the engine's version; with the engine
//! off the PATH, the same rule reports "not available" (exit 3). One rule, `engine = "yamllint"`, `check = "truthy"`,
//! with no yamllint config (yamllint's `default`); example.yaml sets `enabled: yes` on line 2, clean.yaml uses
//! `true`. cctao runs with PATH the folder that gives it yamllint (`cctao_path_for`) then /usr/bin:/bin. Only when
//! yamllint is not on the PATH `cctao` gets (`on_cctao_path`, which runs nothing) is the reported case NOT RUN, with
//! `cctao`'s reason, never PASS; `cctao` must then report the rule not available.

use serde_json::json;

use crate::absent_engine_reason::absent_engine_reason;
use crate::check::Check;
use crate::not_available_pairs::not_available_pairs;
use crate::probed_version::probed_version;
use crate::repo::Repo;
use crate::cctao::cctao_with_path;
use crate::cctao_path_for::cctao_path_for;

const RULE: &str = "yaml-truthy";
const FIX: &str = "Write true or false.";

const RULES: &str = r#"version = 1

[[rule]]
id = "yaml-truthy"
engine = "yamllint"
check = "truthy"
severity = "error"
fix = "Write true or false."
"#;

// Line 2 of EXAMPLE is the truthy value `yes`; CLEAN writes `true`.
const EXAMPLE: &str = "---\nenabled: yes\n";
const CLEAN: &str = "---\nenabled: true\n";

/// A repository holding the rule file, example.yaml and clean.yaml, committed.
fn yaml_repo() -> Repo {
    let repo = Repo::new();
    repo.write("tao.toml", RULES);
    repo.write("example.yaml", EXAMPLE);
    repo.write("clean.yaml", CLEAN);
    repo.commit("yaml files");
    repo
}

#[test]
fn a_yamllint_problem_is_reported_with_its_rule_place_fix_and_version() {
    let mut check = Check::new(
        "a_yamllint_problem_is_reported_with_its_rule_place_fix_and_version",
        "rule yaml-truthy (yamllint, truthy); example.yaml line 2 is `enabled: yes`, clean.yaml writes `true`; PATH \
         is yamllint's folder then /usr/bin:/bin",
    );
    let repo = yaml_repo();
    let args = ["check", "--json"];
    let a = cctao_with_path(&repo, &args, Some(&cctao_path_for(&["yamllint"])));
    check.saw_cctao(&args, a.code, &a.json);
    if let Some(why) = absent_engine_reason(&a, &["yamllint"], RULE) {
        check.not_run(why);
        return;
    }
    assert_eq!(a.code, 1, "a quality error is the refusal (S-15): {:#}\n{}", a.json, a.stderr);
    let f = a.findings();
    assert_eq!(f.len(), 1, "one finding, in example.yaml; clean.yaml passes: {:#}", a.json);
    assert_eq!(f[0]["rule"], RULE);
    assert_eq!(f[0]["engine"], "yamllint");
    assert_eq!(f[0]["path"], "example.yaml");
    assert_eq!(f[0]["line"], 2);
    assert_eq!(f[0]["fix"], FIX);
    let version = probed_version("yamllint", &["--version"], repo.path());
    assert_eq!(a.json["engines"], json!([{ "engine": "yamllint", "version": version }]), "{:#}", a.json);
}

#[test]
fn yamllint_off_path_is_not_available() {
    let mut check = Check::new(
        "yamllint_off_path_is_not_available",
        "rule yaml-truthy (yamllint, truthy) over the same files; cctao runs with PATH /usr/bin:/bin, which holds no \
         yamllint",
    );
    let repo = yaml_repo();
    let args = ["check", "--json"];
    let a = cctao_with_path(&repo, &args, Some("/usr/bin:/bin"));
    check.saw_cctao(&args, a.code, &a.json);
    assert_eq!(a.code, 3, "not available has its own code: {:#}\n{}", a.json, a.stderr);
    assert!(a.findings().is_empty(), "nothing is reported as found: {:#}", a.json);
    assert_eq!(not_available_pairs(&a), vec![(RULE.to_string(), "yamllint".to_string())], "{:#}", a.json);
}

//! Done 9 (mypy) (SPEC-engines-common Done 9): a rule naming the engine reports a known violation, in a file the test
//! made, with its rule, its place and its fix text, and the report lists the engine's version; with the engine off
//! the PATH, the same rule reports "not available" (exit 3). One rule, `engine = "mypy"`, `check = "arg-type"`;
//! example.py passes a str to an int parameter on line 4, clean.py passes an int. cctao runs with PATH the folder that
//! gives it mypy (`cctao_path_for`) then /usr/bin:/bin. Only when mypy is not on the PATH `cctao` gets
//! (`on_cctao_path`, which runs nothing) is the reported case NOT RUN, with `cctao`'s reason, never PASS;
//! `cctao` must then report the rule not available.

use serde_json::json;

use crate::absent_engine_reason::absent_engine_reason;
use crate::check::Check;
use crate::not_available_pairs::not_available_pairs;
use crate::probed_version::probed_version;
use crate::repo::Repo;
use crate::cctao::cctao_with_path;
use crate::cctao_path_for::cctao_path_for;

const RULE: &str = "py-arg-type";
const FIX: &str = "Pass an argument of the expected type.";

const RULES: &str = r#"version = 1

[[rule]]
id = "py-arg-type"
engine = "mypy"
check = "arg-type"
severity = "error"
fix = "Pass an argument of the expected type."
"#;

// Line 4 of EXAMPLE passes a str where f takes an int; CLEAN passes an int.
const EXAMPLE: &str = "def f(x: int) -> int:\n    return x\n\nf(\"s\")\n";
const CLEAN: &str = "def g(x: int) -> int:\n    return x\n\ng(1)\n";

/// A repository holding the rule file, example.py and clean.py, committed.
fn python_repo() -> Repo {
    let repo = Repo::new();
    repo.write("tao.toml", RULES);
    repo.write("example.py", EXAMPLE);
    repo.write("clean.py", CLEAN);
    repo.commit("python files");
    repo
}

#[test]
fn a_mypy_error_is_reported_with_its_rule_place_fix_and_version() {
    let mut check = Check::new(
        "a_mypy_error_is_reported_with_its_rule_place_fix_and_version",
        "rule py-arg-type (mypy, arg-type); example.py line 4 is `f(\"s\")` for `f(x: int)`, clean.py passes an int; \
         PATH is mypy's folder then /usr/bin:/bin",
    );
    let repo = python_repo();
    let args = ["check", "--json"];
    let a = cctao_with_path(&repo, &args, Some(&cctao_path_for(&["mypy"])));
    check.saw_cctao(&args, a.code, &a.json);
    if let Some(why) = absent_engine_reason(&a, &["mypy"], RULE) {
        check.not_run(why);
        return;
    }
    assert_eq!(a.code, 1, "a quality error is the refusal (S-15): {:#}\n{}", a.json, a.stderr);
    let f = a.findings();
    assert_eq!(f.len(), 1, "one finding, in example.py; clean.py passes: {:#}", a.json);
    assert_eq!(f[0]["rule"], RULE);
    assert_eq!(f[0]["engine"], "mypy");
    assert_eq!(f[0]["path"], "example.py");
    assert_eq!(f[0]["line"], 4);
    assert_eq!(f[0]["fix"], FIX);
    let version = probed_version("mypy", &["--version"], repo.path());
    assert_eq!(a.json["engines"], json!([{ "engine": "mypy", "version": version }]), "{:#}", a.json);
}

#[test]
fn mypy_off_path_is_not_available() {
    let mut check = Check::new(
        "mypy_off_path_is_not_available",
        "rule py-arg-type (mypy, arg-type) over the same files; cctao runs with PATH /usr/bin:/bin, which holds no mypy",
    );
    let repo = python_repo();
    let args = ["check", "--json"];
    let a = cctao_with_path(&repo, &args, Some("/usr/bin:/bin"));
    check.saw_cctao(&args, a.code, &a.json);
    assert_eq!(a.code, 3, "not available has its own code: {:#}\n{}", a.json, a.stderr);
    assert!(a.findings().is_empty(), "nothing is reported as found: {:#}", a.json);
    assert_eq!(not_available_pairs(&a), vec![(RULE.to_string(), "mypy".to_string())], "{:#}", a.json);
}

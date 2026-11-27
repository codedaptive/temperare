//! Done 9 (pylint) (SPEC-engines-common Done 9): a rule naming the engine reports a known violation, in a file the
//! test made, with its rule, its place and its fix text, and the report lists the engine's version; with the engine
//! off the PATH, the same rule reports "not available" (exit 3). One rule, `engine = "pylint"`, `check = "C0121"`
//! (comparison to None with `==`); example.py breaks it on line 2, clean.py compares with `is`. Only when pylint is
//! not on the PATH `cctao` gets (`on_cctao_path`, which runs nothing) is the reported case NOT RUN, with
//! `cctao`'s reason, never PASS; `cctao` must then report the rule not available.

use serde_json::json;

use crate::absent_engine_reason::absent_engine_reason;
use crate::check::Check;
use crate::probed_version::probed_version;
use crate::repo::Repo;
use crate::cctao::{cctao, cctao_with_path};

const FIX: &str = "Compare with None using `is`.";

const RULES: &str = r#"version = 1

[[rule]]
id = "py-none-compare"
engine = "pylint"
check = "C0121"
severity = "error"
fix = "Compare with None using `is`."
"#;

// Line 2 of EXAMPLE compares with `== None`; CLEAN compares with `is None`.
const EXAMPLE: &str = "value = None\nif value == None:\n    pass\n";
const CLEAN: &str = "value = None\nif value is None:\n    pass\n";

// git is in /usr/bin; pylint is in neither folder (it is installed elsewhere, such as /opt/homebrew/bin).
const PATH_WITHOUT_ENGINES: &str = "/usr/bin:/bin";

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
fn a_pylint_message_is_reported_with_its_rule_place_fix_and_version() {
    let mut check = Check::new(
        "a_pylint_message_is_reported_with_its_rule_place_fix_and_version",
        "rule py-none-compare (pylint, C0121); example.py line 2 is `if value == None:`, clean.py compares with \
         `is None`",
    );
    let repo = python_repo();
    let args = ["check", "--json"];
    let a = cctao(&repo, &args);
    check.saw_cctao(&args, a.code, &a.json);
    if let Some(why) = absent_engine_reason(&a, &["pylint"], "py-none-compare") {
        check.not_run(why);
        return;
    }
    assert_eq!(a.code, 1, "a quality error is the refusal (S-15): {:#}\n{}", a.json, a.stderr);
    let f = a.findings();
    assert_eq!(f.len(), 1, "one finding, in example.py; clean.py passes: {:#}", a.json);
    assert_eq!(f[0]["rule"], "py-none-compare");
    assert_eq!(f[0]["engine"], "pylint");
    assert_eq!(f[0]["path"], "example.py");
    assert_eq!(f[0]["line"], 2);
    assert_eq!(f[0]["fix"], FIX);
    let version = probed_version("pylint", &["--version"], repo.path());
    assert_eq!(a.json["engines"], json!([{ "engine": "pylint", "version": version }]), "{:#}", a.json);
}

#[test]
fn pylint_off_path_is_not_available() {
    let mut check = Check::new(
        "pylint_off_path_is_not_available",
        "rule py-none-compare (pylint, C0121) over the same files; cctao runs with PATH /usr/bin:/bin, which holds no \
         pylint",
    );
    let repo = python_repo();
    let args = ["check", "--json"];
    let a = cctao_with_path(&repo, &args, Some(PATH_WITHOUT_ENGINES));
    check.saw_cctao(&args, a.code, &a.json);
    assert_eq!(a.code, 3, "not available has its own code: {:#}\n{}", a.json, a.stderr);
    assert!(a.findings().is_empty(), "nothing is reported as found: {:#}", a.json);
    let missing: Vec<(&str, &str)> = a.json["not_available"]
        .as_array()
        .unwrap_or_else(|| panic!("no not_available list: {:#}", a.json))
        .iter()
        .map(|n| (n["rule"].as_str().unwrap_or(""), n["engine"].as_str().unwrap_or("")))
        .collect();
    assert_eq!(missing, vec![("py-none-compare", "pylint")], "{:#}", a.json);
}

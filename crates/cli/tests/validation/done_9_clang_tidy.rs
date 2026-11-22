//! Done 9 (clang-tidy) (SPEC-engines-common Done 9): a rule naming the engine reports a known violation, in a file
//! the test made, with its rule, its place and its fix text, and the report lists the engine's version; with the
//! engine off the PATH, the same rule reports "not available" (exit 3). One rule, `engine = "clang-tidy"`,
//! `check = "readability-braces-around-statements"`; a.c breaks it on line 1, b.c is braced. The repository root
//! holds a compile_flags.txt: without a compile database or compile flags tm reports the rule not available, so the
//! reported case depends on it. Only when clang-tidy is not on the PATH `temperare` gets (`on_tm_path`, which runs
//! nothing) is the reported case NOT RUN, with `temperare`'s reason, never PASS; `temperare` must then report the
//! rule not available.

use serde_json::json;

use crate::absent_engine_reason::absent_engine_reason;
use crate::check::Check;
use crate::probed_version::probed_version;
use crate::repo::Repo;
use crate::tm::{tm, tm_with_path};

const FIX: &str = "Put the statement inside braces.";

const RULES: &str = r#"version = 1

[[rule]]
id = "c-braces"
engine = "clang-tidy"
check = "readability-braces-around-statements"
severity = "error"
fix = "Put the statement inside braces."
"#;

// Line 1 of A_C has an `if` without braces; B_C braces it.
const A_C: &str = "int f(int a) { if (a) return 1; return 0; }\n";
const B_C: &str = "int g(int a) { if (a) { return 1; } return 0; }\n";

// git is in /usr/bin; clang-tidy is in neither folder (it is installed elsewhere, such as /opt/homebrew/bin; Apple's
// /usr/bin holds clang but not clang-tidy).
const PATH_WITHOUT_ENGINES: &str = "/usr/bin:/bin";

/// A repository holding the rule file, compile_flags.txt, a.c and b.c, committed.
fn c_repo() -> Repo {
    let repo = Repo::new();
    repo.write("temperare.toml", RULES);
    repo.write("compile_flags.txt", "-std=c11\n");
    repo.write("a.c", A_C);
    repo.write("b.c", B_C);
    repo.commit("C files");
    repo
}

#[test]
fn a_clang_tidy_warning_is_reported_with_its_rule_place_fix_and_version() {
    let mut check = Check::new(
        "a_clang_tidy_warning_is_reported_with_its_rule_place_fix_and_version",
        "rule c-braces (clang-tidy, readability-braces-around-statements); compile_flags.txt in the root, a.c line 1 \
         has an `if` without braces, b.c braces it",
    );
    let repo = c_repo();
    let args = ["check", "--json"];
    let a = tm(&repo, &args);
    check.saw_tm(&args, a.code, &a.json);
    if let Some(why) = absent_engine_reason(&a, &["clang-tidy"], "c-braces") {
        check.not_run(why);
        return;
    }
    assert_eq!(a.code, 1, "a quality error is the refusal (S-15): {:#}\n{}", a.json, a.stderr);
    let f = a.findings();
    assert_eq!(f.len(), 1, "one finding, in a.c; b.c passes: {:#}", a.json);
    assert_eq!(f[0]["rule"], "c-braces");
    assert_eq!(f[0]["engine"], "clang-tidy");
    assert_eq!(f[0]["path"], "a.c");
    assert_eq!(f[0]["line"], 1);
    assert_eq!(f[0]["fix"], FIX);
    let version = probed_version("clang-tidy", repo.path());
    assert_eq!(a.json["engines"], json!([{ "engine": "clang-tidy", "version": version }]), "{:#}", a.json);
}

#[test]
fn clang_tidy_off_path_is_not_available() {
    let mut check = Check::new(
        "clang_tidy_off_path_is_not_available",
        "rule c-braces (clang-tidy, readability-braces-around-statements) over the same files; tm runs with PATH \
         /usr/bin:/bin, which holds no clang-tidy",
    );
    let repo = c_repo();
    let args = ["check", "--json"];
    let a = tm_with_path(&repo, &args, Some(PATH_WITHOUT_ENGINES));
    check.saw_tm(&args, a.code, &a.json);
    assert_eq!(a.code, 3, "not available has its own code: {:#}\n{}", a.json, a.stderr);
    assert!(a.findings().is_empty(), "nothing is reported as found: {:#}", a.json);
    let missing: Vec<(&str, &str)> = a.json["not_available"]
        .as_array()
        .unwrap_or_else(|| panic!("no not_available list: {:#}", a.json))
        .iter()
        .map(|n| (n["rule"].as_str().unwrap_or(""), n["engine"].as_str().unwrap_or("")))
        .collect();
    assert_eq!(missing, vec![("c-braces", "clang-tidy")], "{:#}", a.json);
}

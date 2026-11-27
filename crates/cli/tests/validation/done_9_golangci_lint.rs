//! Done 9 (golangci-lint) (SPEC-engines-common Done 9): a rule naming the engine reports a known violation, in a file
//! the test made, with its rule, its place and its fix text, and the report lists the engine's version; with the
//! engine off the PATH, the same rule reports "not available" (exit 3). One rule, `engine = "golangci-lint"`,
//! `check = "staticcheck"`, over a Go module the test makes with no dependencies, so nothing is fetched. golangci-lint
//! loads packages with `go`, so cctao needs both. Only when either is not on the PATH `cctao` gets (`on_cctao_path`,
//! which runs nothing) is the reported case NOT RUN, with `cctao`'s reason, never PASS; `cctao` must then
//! report the rule not available.

use serde_json::json;

use crate::absent_engine_reason::absent_engine_reason;
use crate::check::Check;
use crate::probed_version::probed_version;
use crate::repo::Repo;
use crate::cctao::{cctao, cctao_with_path};

const FIX: &str = "Compare two different values; `x == x` is always true.";

const RULES: &str = r#"version = 1

[[rule]]
id = "go-staticcheck"
engine = "golangci-lint"
check = "staticcheck"
severity = "error"
fix = "Compare two different values; `x == x` is always true."
"#;

const GO_MOD: &str = "module example.com/accept\n\ngo 1.22\n";

// Line 5 compares x with itself (staticcheck SA4000, identical expressions); no imports, so nothing is fetched.
const SAME: &str = "package accept\n\n// Same compares x with itself.\nfunc Same(x int) bool {\n\treturn x == x\n}\n";
const CLEAN: &str = "package accept\n\n// Add returns the sum of a and b.\nfunc Add(a, b int) int {\n\treturn a + b\n}\n";

// git is in /usr/bin; neither golangci-lint nor go is in either folder (both are installed elsewhere, such as
// /opt/homebrew/bin).
const PATH_WITHOUT_ENGINES: &str = "/usr/bin:/bin";

/// A repository holding the rule file and the Go module, committed.
fn module_repo() -> Repo {
    let repo = Repo::new();
    repo.write("tao.toml", RULES);
    repo.write("go.mod", GO_MOD);
    repo.write("same.go", SAME);
    repo.write("clean.go", CLEAN);
    repo.commit("a Go module");
    repo
}

#[test]
fn a_golangci_lint_finding_is_reported_with_its_rule_place_fix_and_version() {
    let mut check = Check::new(
        "a_golangci_lint_finding_is_reported_with_its_rule_place_fix_and_version",
        "rule go-staticcheck (golangci-lint, staticcheck); module example.com/accept, same.go line 5 is `x == x`, \
         clean.go adds two ints",
    );
    let repo = module_repo();
    let args = ["check", "--json"];
    let a = cctao(&repo, &args);
    check.saw_cctao(&args, a.code, &a.json);
    if let Some(why) = absent_engine_reason(&a, &["golangci-lint", "go"], "go-staticcheck") {
        check.not_run(why);
        return;
    }
    assert_eq!(a.code, 1, "a quality error is the refusal (S-15): {:#}\n{}", a.json, a.stderr);
    let f = a.findings();
    assert_eq!(f.len(), 1, "one finding, in same.go; clean.go passes: {:#}", a.json);
    assert_eq!(f[0]["rule"], "go-staticcheck");
    assert_eq!(f[0]["engine"], "golangci-lint");
    assert_eq!(f[0]["path"], "same.go");
    assert_eq!(f[0]["line"], 5);
    assert_eq!(f[0]["fix"], FIX);
    let version = probed_version("golangci-lint", &["--version"], repo.path());
    assert_eq!(a.json["engines"], json!([{ "engine": "golangci-lint", "version": version }]), "{:#}", a.json);
}

#[test]
fn golangci_lint_off_path_is_not_available() {
    let mut check = Check::new(
        "golangci_lint_off_path_is_not_available",
        "rule go-staticcheck (golangci-lint, staticcheck) over the same module; cctao runs with PATH /usr/bin:/bin, \
         which holds neither golangci-lint nor go",
    );
    let repo = module_repo();
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
    assert_eq!(missing, vec![("go-staticcheck", "golangci-lint")], "{:#}", a.json);
}

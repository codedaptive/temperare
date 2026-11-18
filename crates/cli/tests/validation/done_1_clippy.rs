//! Done 1: a stock linter through a rule file. "A Rust file that breaks a clippy lint is reported with its rule, its
//! place and its fix text; a clean file passes."

use crate::check::Check;
use crate::repo::Repo;
use crate::tm::tm;

const RULES: &str = r#"version = 1

[[rule]]
id = "no-bool-compare"
engine = "clippy"
check = "clippy::bool_comparison"
severity = "error"
fix = "Use the bool itself: `if x`, not `if x == true`."
"#;

// Its own workspace, so no folder above the temp folder can claim it; no dependencies, so nothing is fetched.
const CARGO_TOML: &str = "[package]\nname = \"accept-one\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n\
                          [dependencies]\n\n[workspace]\n";

fn crate_with(lib: &str) -> Repo {
    let repo = Repo::new();
    repo.write("temperare.toml", RULES);
    repo.write("Cargo.toml", CARGO_TOML);
    repo.write(".gitignore", "target/\n");
    repo.write("src/lib.rs", lib);
    repo.commit("a tiny crate");
    repo
}

#[test]
fn a_clippy_lint_is_reported_with_its_rule_place_and_fix() {
    let mut check = Check::new(
        "a_clippy_lint_is_reported_with_its_rule_place_and_fix",
        "a one-file crate whose src/lib.rs line 2 is `if x == true`; rule no-bool-compare (clippy, \
         clippy::bool_comparison)",
    );
    let repo = crate_with("pub fn yes(x: bool) -> bool {\n    if x == true {\n        return true;\n    }\n    false\n}\n");
    let args = ["check", "--json"];
    let a = tm(&repo, &args);
    check.saw_tm(&args, a.code, &a.json);
    assert_eq!(a.code, 1, "a quality error is the refusal (S-15): {:#}\n{}", a.json, a.stderr);
    let f = a.findings();
    assert_eq!(f.len(), 1, "one finding: {:#}", a.json);
    assert_eq!(f[0]["rule"], "no-bool-compare");
    assert_eq!(f[0]["engine"], "clippy");
    assert_eq!(f[0]["severity"], "error");
    assert_eq!(f[0]["path"], "src/lib.rs");
    assert_eq!(f[0]["line"], 2);
    assert_eq!(f[0]["fix"], "Use the bool itself: `if x`, not `if x == true`.");
}

#[test]
fn a_clean_file_passes() {
    let mut check = Check::new(
        "a_clean_file_passes",
        "a one-file crate whose src/lib.rs returns `x` itself; rule no-bool-compare (clippy, clippy::bool_comparison)",
    );
    let repo = crate_with("pub fn yes(x: bool) -> bool {\n    x\n}\n");
    let args = ["check", "--json"];
    let a = tm(&repo, &args);
    check.saw_tm(&args, a.code, &a.json);
    assert_eq!(a.code, 0, "clean: {:#}\n{}", a.json, a.stderr);
    assert!(a.findings().is_empty(), "no findings: {:#}", a.json);
}

//! A clippy check line, `cargo clippy --all-targets -- -D warnings -A clippy::result_large_err -A
//! clippy::module_inception`, written as one rule: `check = "warnings"` with `params.allow`, which tm runs as
//! `cargo clippy --all-targets` with each allowed lint passed as `--allow=`. Over all targets, so a warning in
//! test code is reported; the two allowed lints are not; a lib warning that two targets compile is one finding; each
//! `what` starts with its lint code. A second rule, every warning with no allow, shows the allowed lints do fire on
//! this crate, so their absence above is the allow's doing.

use crate::check::Check;
use crate::repo::Repo;
use crate::tm::tm;
use serde_json::Value;

const RULES: &str = r#"version = 1

[[rule]]
id = "clippy-line"
engine = "clippy"
check = "warnings"
severity = "error"
fix = "Fix what clippy names, as the clippy -D warnings line asks."
params = { allow = ["clippy::result_large_err", "clippy::module_inception"] }

[[rule]]
id = "every-warning"
engine = "clippy"
check = "warnings"
severity = "warning"
fix = "Fix what clippy names."
"#;

const CARGO_TOML: &str = "[package]\nname = \"clippy-line\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n\
                          [dependencies]\n\n[workspace]\n";

// Line 2: module_inception. Line 9: result_large_err (a 512-byte Err). Line 14: needless_return, in lib code that the
// lib and the lib-test targets both compile. Line 21: bool_comparison, in test code only.
const LIB: &str = "pub mod util {\n    pub mod util {\n        pub fn one() -> u32 {\n            1\n        }\n    }\n}\n\n\
                   pub fn big() -> Result<(), [u8; 512]> {\n    Ok(())\n}\n\n\
                   pub fn is_set(flag: bool) -> bool {\n    return flag;\n}\n\n\
                   #[cfg(test)]\nmod tests {\n    #[test]\n    fn set() {\n        \
                   let set = super::is_set(true) == true;\n        assert!(set);\n    }\n}\n";

/// The rule's findings as (lint code from `what`, line), sorted; every one in src/lib.rs.
fn lints(findings: &[Value], rule: &str) -> Vec<(String, u64)> {
    let mut got: Vec<(String, u64)> = findings
        .iter()
        .filter(|f| f["rule"] == rule)
        .map(|f| {
            assert_eq!(f["path"], "src/lib.rs", "{f:#}");
            let what = f["what"].as_str().unwrap_or("");
            let code = what.split_once(": ").map_or(what, |(code, _)| code).to_string();
            (code, f["line"].as_u64().unwrap_or(0))
        })
        .collect();
    got.sort();
    got
}

#[test]
fn clippy_line_reports_test_code_not_allowed_lints() {
    let mut check = Check::new(
        "clippy_line_reports_test_code_not_allowed_lints",
        "rule clippy-line (clippy warnings, allow result_large_err and module_inception) beside rule every-warning; \
         src/lib.rs: module_inception at 2, result_large_err at 9, needless_return at 14, bool_comparison in test \
         code at 21",
    );
    let repo = Repo::new();
    repo.write("temperare.toml", RULES);
    repo.write("Cargo.toml", CARGO_TOML);
    repo.write(".gitignore", "target/\n");
    repo.write("src/lib.rs", LIB);
    repo.commit("the clippy line on a tiny crate");

    let args = ["check", "--json"];
    let a = tm(&repo, &args);
    check.saw_tm(&args, a.code, &a.json);
    assert_eq!(a.code, 1, "a quality error gets the exit code for a refusal: {:#}\n{}", a.json, a.stderr);
    let f = a.findings();
    assert_eq!(
        lints(f, "clippy-line"),
        vec![(String::from("clippy::bool_comparison"), 21), (String::from("clippy::needless_return"), 14)],
        "the test-code warning and the lib warning once each; the allowed lints not at all: {:#}",
        a.json
    );
    let every: Vec<String> = lints(f, "every-warning").into_iter().map(|(code, _)| code).collect();
    for allowed in ["clippy::module_inception", "clippy::result_large_err"] {
        assert!(every.iter().any(|c| c == allowed), "{allowed} fires without the allow: {:#}", a.json);
    }
}

//! Done 9 (stylelint) (SPEC-engines-common Done 9): a rule naming the engine reports a known violation, in a file the
//! test made, with its rule, its place and its fix text, and the report lists the engine's version; with the engine
//! off the PATH, the same rule reports "not available" (exit 3). One rule, `engine = "stylelint"`, `check =
//! "color-no-invalid-hex"`, whose `config` is stylelint.json turning that rule on, with no `extends`, so nothing is
//! loaded from outside the repository; a.css has an invalid hex colour on line 2, clean.css a valid one. stylelint is
//! a `#!/usr/bin/env node` script, so cctao runs with PATH the folders that give it stylelint and node (`cctao_path_for`)
//! then /usr/bin:/bin; cctao itself checks for stylelint only (its entry's `needs`), not for node. Only when stylelint is
//! not on the PATH `cctao` gets (`on_cctao_path`, which runs nothing) is the reported case NOT RUN, with
//! `cctao`'s reason, never PASS; `cctao` must then report the rule not available.

use serde_json::json;

use crate::absent_engine_reason::absent_engine_reason;
use crate::check::Check;
use crate::not_available_pairs::not_available_pairs;
use crate::probed_version::probed_version;
use crate::repo::Repo;
use crate::cctao::cctao_with_path;
use crate::cctao_path_for::cctao_path_for;

const RULE: &str = "css-invalid-hex";
const FIX: &str = "Write a valid hex colour.";

const RULES: &str = r#"version = 1

[[rule]]
id = "css-invalid-hex"
engine = "stylelint"
check = "color-no-invalid-hex"
config = "stylelint.json"
severity = "error"
fix = "Write a valid hex colour."
"#;

const CONFIG: &str = r#"{"rules":{"color-no-invalid-hex":true}}"#;
// Line 2 of INVALID holds the invalid colour #12345g; CLEAN's colour is valid.
const INVALID: &str = "b { color: #ffffff; }\na { color: #12345g; }\n";
const CLEAN: &str = "a { color: #fff; }\n";

/// A repository holding the rule file, stylelint.json, a.css and clean.css, committed.
fn css_repo() -> Repo {
    let repo = Repo::new();
    repo.write("tao.toml", RULES);
    repo.write("stylelint.json", CONFIG);
    repo.write("a.css", INVALID);
    repo.write("clean.css", CLEAN);
    repo.commit("css files");
    repo
}

#[test]
fn a_stylelint_warning_is_reported_with_its_rule_place_fix_and_version() {
    let mut check = Check::new(
        "a_stylelint_warning_is_reported_with_its_rule_place_fix_and_version",
        "rule css-invalid-hex (stylelint, color-no-invalid-hex, config stylelint.json, no extends); a.css line 2 \
         holds #12345g, clean.css a valid colour; PATH is stylelint's and node's folders then /usr/bin:/bin",
    );
    let repo = css_repo();
    let args = ["check", "--json"];
    let a = cctao_with_path(&repo, &args, Some(&cctao_path_for(&["stylelint", "node"])));
    check.saw_cctao(&args, a.code, &a.json);
    if let Some(why) = absent_engine_reason(&a, &["stylelint"], RULE) {
        check.not_run(why);
        return;
    }
    assert_eq!(a.code, 1, "a quality error is the refusal (S-15): {:#}\n{}", a.json, a.stderr);
    let f = a.findings();
    assert_eq!(f.len(), 1, "one finding, in a.css; clean.css passes: {:#}", a.json);
    assert_eq!(f[0]["rule"], RULE);
    assert_eq!(f[0]["engine"], "stylelint");
    assert_eq!(f[0]["path"], "a.css");
    assert_eq!(f[0]["line"], 2);
    assert_eq!(f[0]["fix"], FIX);
    let version = probed_version("stylelint", &["--version"], repo.path());
    assert_eq!(a.json["engines"], json!([{ "engine": "stylelint", "version": version }]), "{:#}", a.json);
}

#[test]
fn stylelint_off_path_is_not_available() {
    let mut check = Check::new(
        "stylelint_off_path_is_not_available",
        "rule css-invalid-hex (stylelint, color-no-invalid-hex) over the same files; cctao runs with PATH /usr/bin:/bin, \
         which holds no stylelint",
    );
    let repo = css_repo();
    let args = ["check", "--json"];
    let a = cctao_with_path(&repo, &args, Some("/usr/bin:/bin"));
    check.saw_cctao(&args, a.code, &a.json);
    assert_eq!(a.code, 3, "not available has its own code: {:#}\n{}", a.json, a.stderr);
    assert!(a.findings().is_empty(), "nothing is reported as found: {:#}", a.json);
    assert_eq!(not_available_pairs(&a), vec![(RULE.to_string(), "stylelint".to_string())], "{:#}", a.json);
}

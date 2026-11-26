//! Done 9 (vale) (SPEC-engines-common Done 9): a rule naming the engine reports a known violation, in a file the test
//! made, with its rule, its place and its fix text, and the report lists the engine's version; with the engine off
//! the PATH, the same rule reports "not available" (exit 3). One rule, `engine = "vale"`, `check = "Local.Simply"`, a
//! style the test writes: styles/Local/Simply.yml, an existence rule for the word "simply", under the StylesPath the
//! repository's .vale.ini names, whose BasedOnStyles names Local. Nothing is fetched (never `vale sync`). doc.md has
//! "simply" on line 3, clean.md does not. tm runs with PATH the folder that gives it vale (`tm_path_for`) then
//! /usr/bin:/bin. Only when vale is not on the PATH `temperare` gets (`on_tm_path`, which runs nothing) is the
//! reported case NOT RUN, with `temperare`'s reason, never PASS; `temperare` must then report the rule not available.

use serde_json::json;

use crate::absent_engine_reason::absent_engine_reason;
use crate::check::Check;
use crate::not_available_pairs::not_available_pairs;
use crate::probed_version::probed_version;
use crate::repo::Repo;
use crate::tm::tm_with_path;
use crate::tm_path_for::tm_path_for;

const RULE: &str = "prose-no-simply";
const FIX: &str = "Drop the word simply.";

const RULES: &str = r#"version = 1

[[rule]]
id = "prose-no-simply"
engine = "vale"
check = "Local.Simply"
severity = "error"
fix = "Drop the word simply."
"#;

const VALE_INI: &str = "StylesPath = styles\n\n[*.md]\nBasedOnStyles = Local\n";
const SIMPLY: &str = "extends: existence\nmessage: \"Drop '%s'.\"\nlevel: error\ntokens:\n  - simply\n";
// Line 3 of DOC holds "simply"; CLEAN does not.
const DOC: &str = "# Notes\n\nThis is simply wrong.\n";
const CLEAN: &str = "# Notes\n\nThis is plainly right.\n";

/// A repository holding the rule file, .vale.ini, the Local style, doc.md and clean.md, committed.
fn prose_repo() -> Repo {
    let repo = Repo::new();
    repo.write("temperare.toml", RULES);
    repo.write(".vale.ini", VALE_INI);
    repo.write("styles/Local/Simply.yml", SIMPLY);
    repo.write("doc.md", DOC);
    repo.write("clean.md", CLEAN);
    repo.commit("prose and a local style");
    repo
}

#[test]
fn a_vale_alert_is_reported_with_its_rule_place_fix_and_version() {
    let mut check = Check::new(
        "a_vale_alert_is_reported_with_its_rule_place_fix_and_version",
        "rule prose-no-simply (vale, Local.Simply, a style the test writes under .vale.ini's StylesPath); doc.md \
         line 3 says simply, clean.md does not; PATH is vale's folder then /usr/bin:/bin",
    );
    let repo = prose_repo();
    let args = ["check", "--json"];
    let a = tm_with_path(&repo, &args, Some(&tm_path_for(&["vale"])));
    check.saw_tm(&args, a.code, &a.json);
    if let Some(why) = absent_engine_reason(&a, &["vale"], RULE) {
        check.not_run(why);
        return;
    }
    assert_eq!(a.code, 1, "a quality error is the refusal (S-15): {:#}\n{}", a.json, a.stderr);
    let f = a.findings();
    assert_eq!(f.len(), 1, "one finding, in doc.md; clean.md passes: {:#}", a.json);
    assert_eq!(f[0]["rule"], RULE);
    assert_eq!(f[0]["engine"], "vale");
    assert_eq!(f[0]["path"], "doc.md");
    assert_eq!(f[0]["line"], 3);
    assert_eq!(f[0]["fix"], FIX);
    let version = probed_version("vale", &["--version"], repo.path());
    assert_eq!(a.json["engines"], json!([{ "engine": "vale", "version": version }]), "{:#}", a.json);
}

#[test]
fn vale_off_path_is_not_available() {
    let mut check = Check::new(
        "vale_off_path_is_not_available",
        "rule prose-no-simply (vale, Local.Simply) over the same files; tm runs with PATH /usr/bin:/bin, which holds \
         no vale",
    );
    let repo = prose_repo();
    let args = ["check", "--json"];
    let a = tm_with_path(&repo, &args, Some("/usr/bin:/bin"));
    check.saw_tm(&args, a.code, &a.json);
    assert_eq!(a.code, 3, "not available has its own code: {:#}\n{}", a.json, a.stderr);
    assert!(a.findings().is_empty(), "nothing is reported as found: {:#}", a.json);
    assert_eq!(not_available_pairs(&a), vec![(RULE.to_string(), "vale".to_string())], "{:#}", a.json);
}

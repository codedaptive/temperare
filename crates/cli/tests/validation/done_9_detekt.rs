//! Done 9 (detekt) (SPEC-engines-common Done 9): a rule naming the engine reports a known violation, in a file the
//! test made, with its rule, its place and its fix text, and the report lists the engine's version; with the engine
//! off the PATH, the same rule reports "not available" (exit 3). One rule, `engine = "detekt"`, `check =
//! "MagicNumber"`; src/Demo.kt multiplies by 3.14159 on line 4, src/Clean.kt holds a named constant. tm runs with PATH
//! the folder that gives it detekt (`tm_path_for`) then /usr/bin:/bin: Homebrew's detekt is a bash script that
//! finds its Java by an absolute JAVA_HOME, so no Java folder is added. detekt starts a JVM, so nothing here is timed.
//! Only when detekt is not on the PATH `temperare` gets (`on_tm_path`, which runs nothing) is the reported case NOT
//! RUN, with `temperare`'s reason, never PASS; `temperare` must then report the rule not available.

use serde_json::json;

use crate::absent_engine_reason::absent_engine_reason;
use crate::check::Check;
use crate::not_available_pairs::not_available_pairs;
use crate::probed_version::probed_version;
use crate::repo::Repo;
use crate::tm::tm_with_path;
use crate::tm_path_for::tm_path_for;

const RULE: &str = "kt-magic-number";
const FIX: &str = "Replace the magic number with a named constant.";

const RULES: &str = r#"version = 1

[[rule]]
id = "kt-magic-number"
engine = "detekt"
check = "MagicNumber"
severity = "error"
fix = "Replace the magic number with a named constant."
"#;

// Line 4 of DEMO holds the magic number 3.14159; CLEAN names its only number.
const DEMO: &str = "package demo\n\nfun area(r: Double): Double {\n    return r * r * 3.14159\n}\n";
const CLEAN: &str = "package demo\n\nconst val ONE = 1\n";

/// A repository holding the rule file, src/Demo.kt and src/Clean.kt, committed.
fn kotlin_repo() -> Repo {
    let repo = Repo::new();
    repo.write("temperare.toml", RULES);
    repo.write("src/Demo.kt", DEMO);
    repo.write("src/Clean.kt", CLEAN);
    repo.commit("kotlin files");
    repo
}

#[test]
fn a_detekt_finding_is_reported_with_its_rule_place_fix_and_version() {
    let mut check = Check::new(
        "a_detekt_finding_is_reported_with_its_rule_place_fix_and_version",
        "rule kt-magic-number (detekt, MagicNumber); src/Demo.kt line 4 multiplies by 3.14159, src/Clean.kt holds a \
         named constant; PATH is detekt's folder then /usr/bin:/bin",
    );
    let repo = kotlin_repo();
    let args = ["check", "--json"];
    let a = tm_with_path(&repo, &args, Some(&tm_path_for(&["detekt"])));
    check.saw_tm(&args, a.code, &a.json);
    if let Some(why) = absent_engine_reason(&a, &["detekt"], RULE) {
        check.not_run(why);
        return;
    }
    assert_eq!(a.code, 1, "a quality error is the refusal (S-15): {:#}\n{}", a.json, a.stderr);
    let f = a.findings();
    assert_eq!(f.len(), 1, "one finding, in src/Demo.kt; src/Clean.kt passes: {:#}", a.json);
    assert_eq!(f[0]["rule"], RULE);
    assert_eq!(f[0]["engine"], "detekt");
    assert_eq!(f[0]["path"], "src/Demo.kt");
    assert_eq!(f[0]["line"], 4);
    assert_eq!(f[0]["fix"], FIX);
    let version = probed_version("detekt", &["--version"], repo.path());
    assert_eq!(a.json["engines"], json!([{ "engine": "detekt", "version": version }]), "{:#}", a.json);
}

#[test]
fn detekt_off_path_is_not_available() {
    let mut check = Check::new(
        "detekt_off_path_is_not_available",
        "rule kt-magic-number (detekt, MagicNumber) over the same files; tm runs with PATH /usr/bin:/bin, which holds \
         no detekt",
    );
    let repo = kotlin_repo();
    let args = ["check", "--json"];
    let a = tm_with_path(&repo, &args, Some("/usr/bin:/bin"));
    check.saw_tm(&args, a.code, &a.json);
    assert_eq!(a.code, 3, "not available has its own code: {:#}\n{}", a.json, a.stderr);
    assert!(a.findings().is_empty(), "nothing is reported as found: {:#}", a.json);
    assert_eq!(not_available_pairs(&a), vec![(RULE.to_string(), "detekt".to_string())], "{:#}", a.json);
}

//! Done 9 (PMD): a rule naming the engine reports a known violation, in a file the test
//! made, with its rule, its place and its fix text, and the report lists the engine's version; with the engine off the
//! PATH, the same rule reports "not available" (exit 3). One rule, `engine = "pmd"`,
//! `check = "category/java/bestpractices.xml/UnusedPrivateField"`; Foo.java declares a private field it never reads
//! on line 2, Bar.java a package-private one. cctao runs with PATH the folder that gives it pmd (`cctao_path_for`) then
//! /usr/bin:/bin; pmd's launcher finds its own Java (JAVA_HOME, an absolute path), so no java need be on the PATH. The
//! JVM starts slowly: nothing here is timed. Only when pmd is not on the PATH `cctao` gets (`on_cctao_path`, which
//! runs nothing) is the reported case NOT RUN, with `cctao`'s reason, never PASS; `cctao` must then report the
//! rule not available.

use serde_json::json;

use crate::absent_engine_reason::absent_engine_reason;
use crate::check::Check;
use crate::not_available_pairs::not_available_pairs;
use crate::probed_version::probed_version;
use crate::repo::Repo;
use crate::cctao::cctao_with_path;
use crate::cctao_path_for::cctao_path_for;

const RULE: &str = "java-unused-field";
const FIX: &str = "Remove the field or read it.";

const RULES: &str = r#"version = 1

[[rule]]
id = "java-unused-field"
engine = "pmd"
check = "category/java/bestpractices.xml/UnusedPrivateField"
severity = "error"
fix = "Remove the field or read it."
"#;

// Line 2 of EXAMPLE declares a private field nothing reads; CLEAN's field is not private.
const EXAMPLE: &str = "class Foo {\n    private int unused;\n}\n";
const CLEAN: &str = "class Bar {\n    int value;\n}\n";

// git is in /usr/bin; pmd is in neither folder (it is installed elsewhere, such as /opt/homebrew/bin).
const PATH_WITHOUT_ENGINES: &str = "/usr/bin:/bin";

/// A repository holding the rule file, Foo.java and Bar.java, committed.
fn java_repo() -> Repo {
    let repo = Repo::new();
    repo.write("tao.toml", RULES);
    repo.write("Foo.java", EXAMPLE);
    repo.write("Bar.java", CLEAN);
    repo.commit("java files");
    repo
}

#[test]
fn a_pmd_violation_is_reported_with_its_rule_place_fix_and_version() {
    let mut check = Check::new(
        "a_pmd_violation_is_reported_with_its_rule_place_fix_and_version",
        "rule java-unused-field (pmd, category/java/bestpractices.xml/UnusedPrivateField); Foo.java line 2 is \
         `private int unused;`, never read; Bar.java's field is package-private; PATH is pmd's folder then \
         /usr/bin:/bin",
    );
    let repo = java_repo();
    let args = ["check", "--json"];
    let path = cctao_path_for(&["pmd"]);
    let a = cctao_with_path(&repo, &args, Some(&path));
    check.saw_cctao(&args, a.code, &a.json);
    if let Some(why) = absent_engine_reason(&a, &["pmd"], RULE) {
        check.not_run(why);
        return;
    }
    assert_eq!(a.code, 1, "a quality error is the refusal: {:#}\n{}", a.json, a.stderr);
    let f = a.findings();
    assert_eq!(f.len(), 1, "one finding, in Foo.java; Bar.java passes: {:#}", a.json);
    assert_eq!(f[0]["rule"], RULE);
    assert_eq!(f[0]["engine"], "pmd");
    assert_eq!(f[0]["path"], "Foo.java");
    assert_eq!(f[0]["line"], 2);
    assert_eq!(f[0]["fix"], FIX);
    let version = probed_version("pmd", &["--version"], repo.path());
    assert_eq!(a.json["engines"], json!([{ "engine": "pmd", "version": version }]), "{:#}", a.json);
}

#[test]
fn pmd_off_path_is_not_available() {
    let mut check = Check::new(
        "pmd_off_path_is_not_available",
        "rule java-unused-field (pmd, category/java/bestpractices.xml/UnusedPrivateField) over the same files; cctao runs \
         with PATH /usr/bin:/bin, which holds no pmd",
    );
    let repo = java_repo();
    let args = ["check", "--json"];
    let a = cctao_with_path(&repo, &args, Some(PATH_WITHOUT_ENGINES));
    check.saw_cctao(&args, a.code, &a.json);
    assert_eq!(a.code, 3, "not available has its own code: {:#}\n{}", a.json, a.stderr);
    assert!(a.findings().is_empty(), "nothing is reported as found: {:#}", a.json);
    assert_eq!(not_available_pairs(&a), vec![(RULE.to_string(), "pmd".to_string())], "{:#}", a.json);
}

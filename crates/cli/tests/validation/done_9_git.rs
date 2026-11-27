//! Done 9 (git) (SPEC-engines-common Done 9): a rule naming the engine reports a known violation, in a change the test
//! made, with its rule, its place and its fix text, and the report lists the engine's version; with git off the PATH,
//! the same rule reports "not available" (exit 3) for that reason. One rule, `engine = "git"`, `check = "test_ratio"`,
//! with `max_ratio = 2`, `floor = 50` and `test_paths = ["tests/**"]`. The base commit holds only the rule file; the
//! next adds src/lib.rs (10 lines of code) and tests/lib_test.rs (60 lines of test): 60 is past the floor and 6 to 1 is
//! over 2, so the change gets one finding, at the first added line of the test file with the most added lines, which
//! has no column. The reported case runs `cctao check --base <base>` with PATH the folder that gives it git
//! (`cctao_path_for`) then /usr/bin:/bin, so cctao runs the first git on the PATH `probed_version` uses. The off-PATH case
//! gives PATH an empty folder the test made (/usr/bin holds git) and names the rule file with `--rules`, since without
//! it cctao finds the rule file through git, a usage error with no git (exit 2). It gives no `--base`, so with git present
//! the rule would be not available too (ONLY_WITH_BASE_WHY); the case therefore asserts the reason, `git not found on
//! PATH`, which tells an absent git apart. Only when git is not on the PATH `cctao` gets (`on_cctao_path`, which runs
//! nothing) is the reported case NOT RUN, with `cctao`'s reason, never PASS.

use serde_json::json;

use crate::absent_engine_reason::absent_engine_reason;
use crate::check::Check;
use crate::not_available_pairs::not_available_pairs;
use crate::probed_version::probed_version;
use crate::repo::Repo;
use crate::temp_home::TempHome;
use crate::cctao::cctao_with_path;
use crate::cctao_path_for::cctao_path_for;

const RULE: &str = "test-ratio";
const FIX: &str = "Balance the change.";

const RULES: &str = r#"version = 1

[[rule]]
id = "test-ratio"
engine = "git"
check = "test_ratio"
severity = "error"
fix = "Balance the change."

[rule.params]
max_ratio = 2
floor = 50
test_paths = ["tests/**"]
"#;

/// `n` numbered lines, each ending in a newline.
fn lines(n: usize, word: &str) -> String {
    (1..=n).map(|i| format!("// {word} {i}\n")).collect()
}

/// A repository whose base commit holds the rule file and whose next commit adds the code and the test; with the
/// base commit's id.
fn ratio_repo() -> (Repo, String) {
    let repo = Repo::new();
    repo.write("tao.toml", RULES);
    let base = repo.commit("rules");
    repo.write("src/lib.rs", &lines(10, "code"));
    repo.write("tests/lib_test.rs", &lines(60, "test"));
    repo.commit("code and test");
    (repo, base)
}

#[test]
fn a_git_ratio_finding_is_reported_with_its_rule_place_fix_and_version() {
    let mut check = Check::new(
        "a_git_ratio_finding_is_reported_with_its_rule_place_fix_and_version",
        "rule test-ratio (git, test_ratio, max_ratio 2, floor 50, tests/**); the change past the base adds 10 lines \
         to src/lib.rs and 60 to tests/lib_test.rs; `--base <base>`; PATH is git's folder then /usr/bin:/bin",
    );
    let (repo, base) = ratio_repo();
    let args = ["check", "--base", base.as_str(), "--json"];
    let a = cctao_with_path(&repo, &args, Some(&cctao_path_for(&["git"])));
    check.saw_cctao(&args, a.code, &a.json);
    if let Some(why) = absent_engine_reason(&a, &["git"], RULE) {
        check.not_run(why);
        return;
    }
    assert_eq!(a.code, 1, "a quality error is the refusal (S-15): {:#}\n{}", a.json, a.stderr);
    let f = a.findings();
    assert_eq!(f.len(), 1, "one finding for the whole change: {:#}", a.json);
    assert_eq!(f[0]["rule"], RULE);
    assert_eq!(f[0]["engine"], "git");
    assert_eq!(f[0]["path"], "tests/lib_test.rs");
    assert_eq!(f[0]["line"], 1);
    assert!(f[0].get("col").is_none(), "a ratio finding has no column: {:#}", f[0]);
    assert_eq!(f[0]["fix"], FIX);
    let version = probed_version("git", &["--version"], repo.path());
    assert_eq!(a.json["engines"], json!([{ "engine": "git", "version": version }]), "{:#}", a.json);
}

#[test]
fn git_off_path_is_not_available() {
    let mut check = Check::new(
        "git_off_path_is_not_available",
        "rule test-ratio (git, test_ratio) over the same change; `--rules <the rule file>`, no base; cctao runs with \
         PATH an empty folder, which holds no git",
    );
    let (repo, _) = ratio_repo();
    let empty = TempHome::new();
    let rules = repo.path().join("tao.toml").display().to_string();
    let args = ["check", "--rules", rules.as_str(), "--json"];
    let a = cctao_with_path(&repo, &args, Some(&empty.path().display().to_string()));
    check.saw_cctao(&args, a.code, &a.json);
    assert_eq!(a.code, 3, "not available has its own code: {:#}\n{}", a.json, a.stderr);
    assert!(a.findings().is_empty(), "nothing is reported as found: {:#}", a.json);
    assert_eq!(not_available_pairs(&a), vec![(RULE.to_string(), "git".to_string())], "{:#}", a.json);
    // Without --base the rule is not available with git present too (ONLY_WITH_BASE_WHY), so only the reason tells an
    // absent git apart.
    let why = a.json["not_available"].as_array().and_then(|list| list.iter().find(|n| n["rule"] == RULE));
    assert_eq!(why.map(|n| &n["why"]), Some(&json!("git not found on PATH")), "{:#}", a.json);
}

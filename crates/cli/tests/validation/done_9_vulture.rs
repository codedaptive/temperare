//! Acceptance (vulture): a rule naming the engine reports a known violation, in a file the
//! test made, with its rule, its place and its fix text, and the report lists the engine's version; with the engine
//! off the PATH, the same rule reports "not available" (exit 3). One rule, `engine = "vulture"`, `check = "V103"`
//! (unused function); dead.py defines a function nothing calls on line 1, used.py calls its own. cctao runs with PATH
//! the folder that gives it vulture (`cctao_path_folder`) then /usr/bin:/bin, so it gets vulture and git and little
//! else. Only when vulture is not on the PATH `cctao` gets (`on_cctao_path`, which runs nothing) is the reported case
//! NOT RUN, with `cctao`'s reason, never PASS; `cctao` must then report the rule not available.

use serde_json::json;

use crate::absent_engine_reason::absent_engine_reason;
use crate::check::Check;
use crate::probed_version::probed_version;
use crate::repo::Repo;
use crate::cctao::cctao_with_path;
use crate::cctao_path_folder::cctao_path_folder;

const FIX: &str = "Remove the function or call it.";

const RULES: &str = r#"version = 1

[[rule]]
id = "py-dead-function"
engine = "vulture"
check = "V103"
severity = "error"
fix = "Remove the function or call it."
"#;

// Line 1 of DEAD defines a function nothing calls; USED calls the function it defines.
const DEAD: &str = "def dead():\n    pass\n";
const USED: &str = "def used():\n    pass\n\n\nused()\n";

// git is in /usr/bin; vulture is in neither folder (it is installed elsewhere, such as /opt/homebrew/bin).
const PATH_WITHOUT_ENGINES: &str = "/usr/bin:/bin";

/// A repository holding the rule file, dead.py and used.py, committed.
fn python_repo() -> Repo {
    let repo = Repo::new();
    repo.write("tao.toml", RULES);
    repo.write("dead.py", DEAD);
    repo.write("used.py", USED);
    repo.commit("python files");
    repo
}

#[test]
fn a_vulture_finding_is_reported_with_its_rule_place_fix_and_version() {
    let mut check = Check::new(
        "a_vulture_finding_is_reported_with_its_rule_place_fix_and_version",
        "rule py-dead-function (vulture, V103); dead.py line 1 defines a function nothing calls, used.py calls its \
         own; PATH is vulture's folder then /usr/bin:/bin",
    );
    let repo = python_repo();
    let args = ["check", "--json"];
    let path = cctao_path_folder("vulture")
        .map_or(PATH_WITHOUT_ENGINES.to_string(), |dir| format!("{dir}:{PATH_WITHOUT_ENGINES}"));
    let a = cctao_with_path(&repo, &args, Some(&path));
    check.saw_cctao(&args, a.code, &a.json);
    if let Some(why) = absent_engine_reason(&a, &["vulture"], "py-dead-function") {
        check.not_run(why);
        return;
    }
    assert_eq!(a.code, 1, "a quality error is the refusal: {:#}\n{}", a.json, a.stderr);
    let f = a.findings();
    assert_eq!(f.len(), 1, "one finding, in dead.py; used.py passes: {:#}", a.json);
    assert_eq!(f[0]["rule"], "py-dead-function");
    assert_eq!(f[0]["engine"], "vulture");
    assert_eq!(f[0]["path"], "dead.py");
    assert_eq!(f[0]["line"], 1);
    assert_eq!(f[0]["fix"], FIX);
    let version = probed_version("vulture", &["--version"], repo.path());
    assert_eq!(a.json["engines"], json!([{ "engine": "vulture", "version": version }]), "{:#}", a.json);
}

#[test]
fn vulture_off_path_is_not_available() {
    let mut check = Check::new(
        "vulture_off_path_is_not_available",
        "rule py-dead-function (vulture, V103) over the same files; cctao runs with PATH /usr/bin:/bin, which holds no \
         vulture",
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
    assert_eq!(missing, vec![("py-dead-function", "vulture")], "{:#}", a.json);
}

//! Done 9 (Periphery) (SPEC-engines-common Done 9): a rule naming the engine reports a known violation, in a file the
//! test made, with its rule, its place and its fix text, and the report lists the engine's version; with the engine
//! off the PATH, the same rule reports "not available" (exit 3). One rule, `engine = "periphery"`, `check = "unused"`,
//! over a Swift package the test makes at the repository root with no dependencies, so nothing is fetched: target
//! Probe's main.swift declares a function nothing calls on line 1, then prints. Periphery needs swift, which cctao
//! builds the package with. cctao is not given the caller's PATH, only the folder that gives it periphery
//! (`cctao_path_folder`) then /usr/bin:/bin, so the swift cctao runs is /usr/bin/swift unless that folder holds one. Only
//! when periphery is not on the PATH `cctao` gets, or /usr/bin/swift is not there (`on_cctao_path`, which runs
//! nothing), is the reported case NOT RUN, with `cctao`'s reason, never PASS; `cctao` must then report the
//! rule not available.

use serde_json::json;

use crate::absent_engine_reason::absent_engine_reason;
use crate::check::Check;
use crate::probed_version::probed_version;
use crate::repo::Repo;
use crate::cctao::cctao_with_path;
use crate::cctao_path_folder::cctao_path_folder;

const FIX: &str = "Remove the declaration or use it.";

const RULES: &str = r#"version = 1

[[rule]]
id = "swift-unused"
engine = "periphery"
check = "unused"
severity = "error"
fix = "Remove the declaration or use it."
"#;

const MANIFEST: &str = "// swift-tools-version: 6.0\nimport PackageDescription\nlet package = Package(name: \"Probe\", \
                        targets: [.executableTarget(name: \"Probe\")])\n";

// Line 1 of MAIN declares a function nothing calls (Periphery reports it at line 1, column 6); line 2 is top-level
// code, which Periphery does not report.
const MAIN: &str = "func unusedHelper() {}\nprint(\"hello\")\n";

// git and Apple's swift are in /usr/bin; periphery is in neither folder (it is installed elsewhere, such as
// /opt/homebrew/bin).
const PATH_WITHOUT_ENGINES: &str = "/usr/bin:/bin";

/// A repository holding the rule file and the Swift package, committed.
fn package_repo() -> Repo {
    let repo = Repo::new();
    repo.write("tao.toml", RULES);
    repo.write("Package.swift", MANIFEST);
    repo.write("Sources/Probe/main.swift", MAIN);
    repo.commit("a Swift package");
    repo
}

#[test]
fn a_periphery_result_is_reported_with_its_rule_place_fix_and_version() {
    let mut check = Check::new(
        "a_periphery_result_is_reported_with_its_rule_place_fix_and_version",
        "rule swift-unused (periphery, unused); package Probe, Sources/Probe/main.swift line 1 declares a function \
         nothing calls, line 2 prints; PATH is periphery's folder then /usr/bin:/bin",
    );
    let repo = package_repo();
    let args = ["check", "--json"];
    let path = cctao_path_folder("periphery")
        .map_or(PATH_WITHOUT_ENGINES.to_string(), |dir| format!("{dir}:{PATH_WITHOUT_ENGINES}"));
    let a = cctao_with_path(&repo, &args, Some(&path));
    check.saw_cctao(&args, a.code, &a.json);
    // /usr/bin/swift is the swift cctao gets unless the periphery folder holds one.
    if let Some(why) = absent_engine_reason(&a, &["periphery", "/usr/bin/swift"], "swift-unused") {
        check.not_run(why);
        return;
    }
    // The rule is quality (no kind given) at severity error, and a kept quality error is exit 1 (S-15), whichever
    // engine found it.
    assert_eq!(a.code, 1, "a quality error is the refusal (S-15): {:#}\n{}", a.json, a.stderr);
    let f = a.findings();
    assert_eq!(f.len(), 1, "one finding, main.swift line 1; its top-level print is not one: {:#}", a.json);
    assert_eq!(f[0]["rule"], "swift-unused");
    assert_eq!(f[0]["engine"], "periphery");
    assert_eq!(f[0]["path"], "Sources/Probe/main.swift");
    assert_eq!(f[0]["line"], 1);
    assert_eq!(f[0]["fix"], FIX);
    let version = probed_version("periphery", &["version"], repo.path());
    assert_eq!(a.json["engines"], json!([{ "engine": "periphery", "version": version }]), "{:#}", a.json);
}

#[test]
fn periphery_off_path_is_not_available() {
    let mut check = Check::new(
        "periphery_off_path_is_not_available",
        "rule swift-unused (periphery, unused) over the same package; cctao runs with PATH /usr/bin:/bin, which holds \
         swift but no periphery",
    );
    let repo = package_repo();
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
    assert_eq!(missing, vec![("swift-unused", "periphery")], "{:#}", a.json);
}

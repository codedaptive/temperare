//! Done 9 (dart): a rule naming the engine reports a known violation, in a file the test
//! made, with its rule, its place and its fix text, and the report lists the engine's version; with the engine off
//! the PATH, the same rule reports "not available" (exit 3). One rule, `engine = "dart"`, `check =
//! "unused_local_variable"` (a diagnostic, so no analysis_options.yaml need enable it); lib/a.dart declares a variable
//! it never uses on line 2, lib/clean.dart prints its own. The package's pubspec.yaml has no dependencies, and
//! `dart analyze` never runs `pub get`, so nothing is fetched. cctao runs with PATH the folder that gives it dart
//! (`cctao_path_for`) then /usr/bin:/bin. The dart VM is slow to start, so nothing here is timed. Only when dart is
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

const RULE: &str = "dart-unused-local";
const FIX: &str = "Use the variable or remove it.";

const RULES: &str = r#"version = 1

[[rule]]
id = "dart-unused-local"
engine = "dart"
check = "unused_local_variable"
severity = "error"
fix = "Use the variable or remove it."
"#;

const PUBSPEC: &str = "name: probe\nenvironment:\n  sdk: ^3.0.0\n";
// Line 2 of UNUSED declares a variable nothing reads; CLEAN prints the one it declares.
const UNUSED: &str = "void main() {\n  var unused = 1;\n}\n";
const CLEAN: &str = "void clean() {\n  var used = 1;\n  print(used);\n}\n";

/// A repository holding the rule file, pubspec.yaml, lib/a.dart and lib/clean.dart, committed.
fn dart_repo() -> Repo {
    let repo = Repo::new();
    repo.write("tao.toml", RULES);
    repo.write("pubspec.yaml", PUBSPEC);
    repo.write("lib/a.dart", UNUSED);
    repo.write("lib/clean.dart", CLEAN);
    repo.commit("dart package");
    repo
}

#[test]
fn a_dart_diagnostic_is_reported_with_its_rule_place_fix_and_version() {
    let mut check = Check::new(
        "a_dart_diagnostic_is_reported_with_its_rule_place_fix_and_version",
        "rule dart-unused-local (dart, unused_local_variable); pubspec.yaml with no dependencies; lib/a.dart line 2 \
         declares a variable it never reads, lib/clean.dart prints its own; PATH is dart's folder then /usr/bin:/bin",
    );
    let repo = dart_repo();
    let args = ["check", "--json"];
    let a = cctao_with_path(&repo, &args, Some(&cctao_path_for(&["dart"])));
    check.saw_cctao(&args, a.code, &a.json);
    if let Some(why) = absent_engine_reason(&a, &["dart"], RULE) {
        check.not_run(why);
        return;
    }
    assert_eq!(a.code, 1, "a quality error is the refusal: {:#}\n{}", a.json, a.stderr);
    let f = a.findings();
    assert_eq!(f.len(), 1, "one finding, in lib/a.dart; lib/clean.dart passes: {:#}", a.json);
    assert_eq!(f[0]["rule"], RULE);
    assert_eq!(f[0]["engine"], "dart");
    assert_eq!(f[0]["path"], "lib/a.dart");
    assert_eq!(f[0]["line"], 2);
    assert_eq!(f[0]["fix"], FIX);
    let version = probed_version("dart", &["--suppress-analytics", "--version"], repo.path());
    assert_eq!(a.json["engines"], json!([{ "engine": "dart", "version": version }]), "{:#}", a.json);
}

#[test]
fn dart_off_path_is_not_available() {
    let mut check = Check::new(
        "dart_off_path_is_not_available",
        "rule dart-unused-local (dart, unused_local_variable) over the same package; cctao runs with PATH /usr/bin:/bin, \
         which holds no dart",
    );
    let repo = dart_repo();
    let args = ["check", "--json"];
    let a = cctao_with_path(&repo, &args, Some("/usr/bin:/bin"));
    check.saw_cctao(&args, a.code, &a.json);
    assert_eq!(a.code, 3, "not available has its own code: {:#}\n{}", a.json, a.stderr);
    assert!(a.findings().is_empty(), "nothing is reported as found: {:#}", a.json);
    assert_eq!(not_available_pairs(&a), vec![(RULE.to_string(), "dart".to_string())], "{:#}", a.json);
}

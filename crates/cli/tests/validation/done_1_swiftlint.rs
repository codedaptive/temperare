//! Done 1 (SwiftLint) (S-31, SPEC-accept WO-orc-accept-2): the stock linter check with SwiftLint run for real. One
//! rule, `engine = "swiftlint"`, `check = "force_cast"`: a Swift file holding `as!` is reported with its rule, path,
//! line and fix, exit 1; a Swift file without it passes, exit 0. The repository holds no `.swiftlint.yml`. Only when
//! swiftlint is not on the PATH `tm` gets (`on_tm_path`, which runs nothing) is a case NOT RUN, with `tm`'s reason,
//! never PASS; `tm` must then report the rule not available. With swiftlint there, the case asserts as usual.

use crate::check::Check;
use crate::on_tm_path::on_tm_path;
use crate::repo::Repo;
use crate::tm::{tm, Answer};

const FIX: &str = "Use `as?` and handle nil.";

const RULES: &str = r#"version = 1

[[rule]]
id = "no-force-cast"
engine = "swiftlint"
check = "force_cast"
severity = "error"
fix = "Use `as?` and handle nil."
"#;

// Line 2 of CAST force-casts with `as!`; SAFE casts the same value with `as?`.
const CAST: &str = "let value: Any = 1\nlet number = value as! Int\n";
const SAFE: &str = "let value: Any = 1\nlet number = value as? Int\n";

/// A repository holding the rule file and each (path, text), committed.
fn repo_with(files: &[(&str, &str)]) -> Repo {
    let repo = Repo::new();
    repo.write("temperare.toml", RULES);
    for (path, text) in files {
        repo.write(path, text);
    }
    repo.commit("swift files");
    repo
}

/// NOT RUN's reason when swiftlint is not on the PATH `tm` gets: `tm`'s own reason for reporting the rule not
/// available, which it must then give (panics when it does not). None when swiftlint is there.
fn not_run_reason(a: &Answer) -> Option<String> {
    if on_tm_path("swiftlint") {
        return None;
    }
    let why = a.json["not_available"]
        .as_array()
        .and_then(|list| list.iter().find(|n| n["rule"] == "no-force-cast"))
        .and_then(|n| n["why"].as_str())
        .unwrap_or_else(|| {
            panic!("swiftlint is not on the PATH tm gets, yet tm did not report no-force-cast: {:#}", a.json)
        });
    Some(format!("tm: {why}"))
}

#[test]
fn a_force_cast_is_reported_with_its_rule_place_and_fix() {
    let mut check = Check::new(
        "a_force_cast_is_reported_with_its_rule_place_and_fix",
        "rule no-force-cast (swiftlint, force_cast); Sources/Cast.swift line 2 holds `as!`, Sources/Safe.swift casts \
         with `as?`",
    );
    let repo = repo_with(&[("Sources/Cast.swift", CAST), ("Sources/Safe.swift", SAFE)]);
    let args = ["check", "--json"];
    let a = tm(&repo, &args);
    check.saw_tm(&args, a.code, &a.json);
    if let Some(why) = not_run_reason(&a) {
        check.not_run(why);
        return;
    }
    assert_eq!(a.code, 1, "a quality error is the refusal (S-15): {:#}\n{}", a.json, a.stderr);
    let f = a.findings();
    assert_eq!(f.len(), 1, "one finding, in Cast.swift; Safe.swift passes: {:#}", a.json);
    assert_eq!(f[0]["rule"], "no-force-cast");
    assert_eq!(f[0]["engine"], "swiftlint");
    assert_eq!(f[0]["path"], "Sources/Cast.swift");
    assert_eq!(f[0]["line"], 2);
    assert_eq!(f[0]["fix"], FIX);
}

#[test]
fn a_file_without_a_force_cast_passes() {
    let mut check = Check::new(
        "a_file_without_a_force_cast_passes",
        "rule no-force-cast (swiftlint, force_cast); the only Swift file is Sources/Safe.swift, which casts with \
         `as?`",
    );
    let repo = repo_with(&[("Sources/Safe.swift", SAFE)]);
    let args = ["check", "--json"];
    let a = tm(&repo, &args);
    check.saw_tm(&args, a.code, &a.json);
    if let Some(why) = not_run_reason(&a) {
        check.not_run(why);
        return;
    }
    assert_eq!(a.code, 0, "clean: {:#}\n{}", a.json, a.stderr);
    assert!(a.findings().is_empty(), "no findings: {:#}", a.json);
}

//! Done 9 (hadolint) (SPEC-engines-common Done 9): a rule naming the engine reports a known violation, in a file the
//! test made, with its rule, its place and its fix text, and the report lists the engine's version; with the engine
//! off the PATH, the same rule reports "not available" (exit 3). One rule, `engine = "hadolint"`, `check = "DL3006"`
//! (an untagged image); Dockerfile's line 2 is `FROM ubuntu`, web.Dockerfile tags its image. tm runs with PATH the
//! folder that gives it hadolint (`tm_path_for`) then /usr/bin:/bin. Only when hadolint is not on the PATH
//! `temperare` gets (`on_tm_path`, which runs nothing) is the reported case NOT RUN, with `temperare`'s reason, never
//! PASS; `temperare` must then report the rule not available.

use serde_json::json;

use crate::absent_engine_reason::absent_engine_reason;
use crate::check::Check;
use crate::not_available_pairs::not_available_pairs;
use crate::probed_version::probed_version;
use crate::repo::Repo;
use crate::tm::tm_with_path;
use crate::tm_path_for::tm_path_for;

const RULE: &str = "docker-tag";
const FIX: &str = "Tag the image.";

const RULES: &str = r#"version = 1

[[rule]]
id = "docker-tag"
engine = "hadolint"
check = "DL3006"
severity = "error"
fix = "Tag the image."
"#;

// Line 2 of EXAMPLE names an image with no tag; CLEAN tags it.
const EXAMPLE: &str = "# the base image\nFROM ubuntu\n";
const CLEAN: &str = "FROM ubuntu:22.04\n";

/// A repository holding the rule file, Dockerfile and web.Dockerfile, committed.
fn docker_repo() -> Repo {
    let repo = Repo::new();
    repo.write("temperare.toml", RULES);
    repo.write("Dockerfile", EXAMPLE);
    repo.write("web.Dockerfile", CLEAN);
    repo.commit("dockerfiles");
    repo
}

#[test]
fn a_hadolint_warning_is_reported_with_its_rule_place_fix_and_version() {
    let mut check = Check::new(
        "a_hadolint_warning_is_reported_with_its_rule_place_fix_and_version",
        "rule docker-tag (hadolint, DL3006); Dockerfile line 2 is `FROM ubuntu`, web.Dockerfile is \
         `FROM ubuntu:22.04`; PATH is hadolint's folder then /usr/bin:/bin",
    );
    let repo = docker_repo();
    let args = ["check", "--json"];
    let a = tm_with_path(&repo, &args, Some(&tm_path_for(&["hadolint"])));
    check.saw_tm(&args, a.code, &a.json);
    if let Some(why) = absent_engine_reason(&a, &["hadolint"], RULE) {
        check.not_run(why);
        return;
    }
    assert_eq!(a.code, 1, "a quality error is the refusal (S-15): {:#}\n{}", a.json, a.stderr);
    let f = a.findings();
    assert_eq!(f.len(), 1, "one finding, in Dockerfile; web.Dockerfile passes: {:#}", a.json);
    assert_eq!(f[0]["rule"], RULE);
    assert_eq!(f[0]["engine"], "hadolint");
    assert_eq!(f[0]["path"], "Dockerfile");
    assert_eq!(f[0]["line"], 2);
    assert_eq!(f[0]["fix"], FIX);
    let version = probed_version("hadolint", &["--version"], repo.path());
    assert_eq!(a.json["engines"], json!([{ "engine": "hadolint", "version": version }]), "{:#}", a.json);
}

#[test]
fn hadolint_off_path_is_not_available() {
    let mut check = Check::new(
        "hadolint_off_path_is_not_available",
        "rule docker-tag (hadolint, DL3006) over the same files; tm runs with PATH /usr/bin:/bin, which holds no \
         hadolint",
    );
    let repo = docker_repo();
    let args = ["check", "--json"];
    let a = tm_with_path(&repo, &args, Some("/usr/bin:/bin"));
    check.saw_tm(&args, a.code, &a.json);
    assert_eq!(a.code, 3, "not available has its own code: {:#}\n{}", a.json, a.stderr);
    assert!(a.findings().is_empty(), "nothing is reported as found: {:#}", a.json);
    assert_eq!(not_available_pairs(&a), vec![(RULE.to_string(), "hadolint".to_string())], "{:#}", a.json);
}

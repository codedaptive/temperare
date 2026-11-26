//! Done 9 (RuboCop) (SPEC-engines-common Done 9): a rule naming the engine reports a known violation, in a file the
//! test made, with its rule, its place and its fix text, and the report lists the engine's version; with the engine
//! off the PATH, the same rule reports "not available" (exit 3). One rule, `engine = "rubocop"`,
//! `check = "Lint/UselessAssignment"`, under rubocop's stock config (the repository has no .rubocop.yml); example.rb
//! assigns a local it never reads on line 4, clean.rb assigns none. tm runs with PATH the folder that gives it
//! rubocop (`tm_path_for`) then /usr/bin:/bin; rubocop's launcher names its ruby by absolute path. Only when rubocop
//! is not on the PATH `temperare` gets (`on_tm_path`, which runs nothing) is the reported case NOT RUN, with
//! `temperare`'s reason, never PASS; `temperare` must then report the rule not available.

use serde_json::json;

use crate::absent_engine_reason::absent_engine_reason;
use crate::check::Check;
use crate::not_available_pairs::not_available_pairs;
use crate::probed_version::probed_version;
use crate::repo::Repo;
use crate::tm::tm_with_path;
use crate::tm_path_for::tm_path_for;

const RULE: &str = "rb-useless-assignment";
const FIX: &str = "Remove the assignment or use the variable.";

const RULES: &str = r#"version = 1

[[rule]]
id = "rb-useless-assignment"
engine = "rubocop"
check = "Lint/UselessAssignment"
severity = "error"
fix = "Remove the assignment or use the variable."
"#;

// Line 4 of EXAMPLE assigns `unused`, never read; CLEAN assigns nothing.
const EXAMPLE: &str = "# frozen_string_literal: true\n\ndef sample\n  unused = 1\n  nil\nend\n";
const CLEAN: &str = "# frozen_string_literal: true\n\ndef other\n  nil\nend\n";

// git is in /usr/bin; rubocop is in neither folder (it is installed elsewhere, such as /opt/homebrew/bin).
const PATH_WITHOUT_ENGINES: &str = "/usr/bin:/bin";

/// A repository holding the rule file, example.rb and clean.rb, committed.
fn ruby_repo() -> Repo {
    let repo = Repo::new();
    repo.write("temperare.toml", RULES);
    repo.write("example.rb", EXAMPLE);
    repo.write("clean.rb", CLEAN);
    repo.commit("ruby files");
    repo
}

#[test]
fn a_rubocop_offense_is_reported_with_its_rule_place_fix_and_version() {
    let mut check = Check::new(
        "a_rubocop_offense_is_reported_with_its_rule_place_fix_and_version",
        "rule rb-useless-assignment (rubocop, Lint/UselessAssignment), no .rubocop.yml; example.rb line 4 is \
         `unused = 1`, never read; clean.rb assigns nothing; PATH is rubocop's folder then /usr/bin:/bin",
    );
    let repo = ruby_repo();
    let args = ["check", "--json"];
    let path = tm_path_for(&["rubocop"]);
    let a = tm_with_path(&repo, &args, Some(&path));
    check.saw_tm(&args, a.code, &a.json);
    if let Some(why) = absent_engine_reason(&a, &["rubocop"], RULE) {
        check.not_run(why);
        return;
    }
    assert_eq!(a.code, 1, "a quality error is the refusal (S-15): {:#}\n{}", a.json, a.stderr);
    let f = a.findings();
    assert_eq!(f.len(), 1, "one finding, in example.rb; clean.rb passes: {:#}", a.json);
    assert_eq!(f[0]["rule"], RULE);
    assert_eq!(f[0]["engine"], "rubocop");
    assert_eq!(f[0]["path"], "example.rb");
    assert_eq!(f[0]["line"], 4);
    assert_eq!(f[0]["fix"], FIX);
    let version = probed_version("rubocop", &["--force-default-config", "--version"], repo.path());
    assert_eq!(a.json["engines"], json!([{ "engine": "rubocop", "version": version }]), "{:#}", a.json);
}

#[test]
fn rubocop_off_path_is_not_available() {
    let mut check = Check::new(
        "rubocop_off_path_is_not_available",
        "rule rb-useless-assignment (rubocop, Lint/UselessAssignment) over the same files; tm runs with PATH \
         /usr/bin:/bin, which holds no rubocop",
    );
    let repo = ruby_repo();
    let args = ["check", "--json"];
    let a = tm_with_path(&repo, &args, Some(PATH_WITHOUT_ENGINES));
    check.saw_tm(&args, a.code, &a.json);
    assert_eq!(a.code, 3, "not available has its own code: {:#}\n{}", a.json, a.stderr);
    assert!(a.findings().is_empty(), "nothing is reported as found: {:#}", a.json);
    assert_eq!(not_available_pairs(&a), vec![(RULE.to_string(), "rubocop".to_string())], "{:#}", a.json);
}

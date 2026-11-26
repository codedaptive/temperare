//! Done 9 (ShellCheck) (SPEC-engines-common Done 9): a rule naming the engine reports a known violation, in a file the
//! test made, with its rule, its place and its fix text, and the report lists the engine's version; with the engine
//! off the PATH, the same rule reports "not available" (exit 3). One rule, `engine = "shellcheck"`, `check = "SC2086"`
//! (an unquoted expansion); run.sh's line 3 is `echo $1`, quoted.sh quotes it. Both start with a `#!/bin/sh` shebang:
//! a script shellcheck cannot parse gets an error-level SC1xxx code instead, which fails the rule. tm runs with PATH
//! the folder that gives it shellcheck (`tm_path_for`) then /usr/bin:/bin. Only when shellcheck is not on the PATH
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

const RULE: &str = "shell-quote";
const FIX: &str = "Quote the expansion.";

const RULES: &str = r#"version = 1

[[rule]]
id = "shell-quote"
engine = "shellcheck"
check = "SC2086"
severity = "error"
fix = "Quote the expansion."
"#;

// Line 3 of EXAMPLE expands $1 unquoted; CLEAN quotes it.
const EXAMPLE: &str = "#!/bin/sh\n# print the first argument\necho $1\n";
const CLEAN: &str = "#!/bin/sh\n# print the first argument\necho \"$1\"\n";

/// A repository holding the rule file, run.sh and quoted.sh, committed.
fn shell_repo() -> Repo {
    let repo = Repo::new();
    repo.write("temperare.toml", RULES);
    repo.write("run.sh", EXAMPLE);
    repo.write("quoted.sh", CLEAN);
    repo.commit("scripts");
    repo
}

#[test]
fn a_shellcheck_warning_is_reported_with_its_rule_place_fix_and_version() {
    let mut check = Check::new(
        "a_shellcheck_warning_is_reported_with_its_rule_place_fix_and_version",
        "rule shell-quote (shellcheck, SC2086); run.sh line 3 is `echo $1`, quoted.sh is `echo \"$1\"`, both under \
         `#!/bin/sh`; PATH is shellcheck's folder then /usr/bin:/bin",
    );
    let repo = shell_repo();
    let args = ["check", "--json"];
    let a = tm_with_path(&repo, &args, Some(&tm_path_for(&["shellcheck"])));
    check.saw_tm(&args, a.code, &a.json);
    if let Some(why) = absent_engine_reason(&a, &["shellcheck"], RULE) {
        check.not_run(why);
        return;
    }
    assert_eq!(a.code, 1, "a quality error is the refusal (S-15): {:#}\n{}", a.json, a.stderr);
    let f = a.findings();
    assert_eq!(f.len(), 1, "one finding, in run.sh; quoted.sh passes: {:#}", a.json);
    assert_eq!(f[0]["rule"], RULE);
    assert_eq!(f[0]["engine"], "shellcheck");
    assert_eq!(f[0]["path"], "run.sh");
    assert_eq!(f[0]["line"], 3);
    assert_eq!(f[0]["fix"], FIX);
    let version = probed_version("shellcheck", &["--version"], repo.path());
    assert_eq!(a.json["engines"], json!([{ "engine": "shellcheck", "version": version }]), "{:#}", a.json);
}

#[test]
fn shellcheck_off_path_is_not_available() {
    let mut check = Check::new(
        "shellcheck_off_path_is_not_available",
        "rule shell-quote (shellcheck, SC2086) over the same files; tm runs with PATH /usr/bin:/bin, which holds no \
         shellcheck",
    );
    let repo = shell_repo();
    let args = ["check", "--json"];
    let a = tm_with_path(&repo, &args, Some("/usr/bin:/bin"));
    check.saw_tm(&args, a.code, &a.json);
    assert_eq!(a.code, 3, "not available has its own code: {:#}\n{}", a.json, a.stderr);
    assert!(a.findings().is_empty(), "nothing is reported as found: {:#}", a.json);
    assert_eq!(not_available_pairs(&a), vec![(RULE.to_string(), "shellcheck".to_string())], "{:#}", a.json);
}

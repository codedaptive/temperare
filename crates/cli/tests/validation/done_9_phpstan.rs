//! Done 9 (phpstan) (SPEC-engines-common Done 9): a rule naming the engine reports a known violation, in a file the
//! test made, with its rule, its place and its fix text, and the report lists the engine's version; with the engine
//! off the PATH, the same rule reports "not available" (exit 3). One rule, `engine = "phpstan"`, `check = "5"` (the
//! level), over a phpstan.neon of its own (cctao runs PHPStan only with the repository's config); src/example.php passes
//! a string to an int parameter on line 3, src/clean.php passes a string to a string one. The phpstan launcher needs
//! php, so cctao runs with PATH the folders that give it phpstan and php (`cctao_path_for`) then /usr/bin:/bin. Only
//! when either is not on the PATH `cctao` gets (`on_cctao_path`, which runs nothing) is the reported case NOT RUN,
//! with `cctao`'s reason, never PASS; `cctao` must then report the rule not available.

use serde_json::json;

use crate::absent_engine_reason::absent_engine_reason;
use crate::check::Check;
use crate::not_available_pairs::not_available_pairs;
use crate::probed_version::probed_version;
use crate::repo::Repo;
use crate::cctao::cctao_with_path;
use crate::cctao_path_for::cctao_path_for;

const RULE: &str = "php-argument-type";
const FIX: &str = "Pass an integer.";

const RULES: &str = r#"version = 1

[[rule]]
id = "php-argument-type"
engine = "phpstan"
check = "5"
severity = "error"
fix = "Pass an integer."
"#;

const CONFIG: &str = "parameters:\n    level: 0\n    paths:\n        - src\n";

// Line 3 of EXAMPLE passes a string where takeInt takes an int; CLEAN passes a string to a string parameter.
const EXAMPLE: &str = "<?php\nfunction takeInt(int $value): void {}\ntakeInt('wrong');\n";
const CLEAN: &str = "<?php\nfunction takeText(string $value): void {}\ntakeText('right');\n";

// The programs cctao needs on its PATH, as PHPSTAN's `needs` lists them: phpstan and the php its launcher starts.
const PROGRAMS: [&str; 2] = ["phpstan", "php"];

/// A repository holding the rule file, phpstan.neon, src/example.php and src/clean.php, committed.
fn php_repo() -> Repo {
    let repo = Repo::new();
    repo.write("tao.toml", RULES);
    repo.write("phpstan.neon", CONFIG);
    repo.write("src/example.php", EXAMPLE);
    repo.write("src/clean.php", CLEAN);
    repo.commit("php files");
    repo
}

#[test]
fn a_phpstan_error_is_reported_with_its_rule_place_fix_and_version() {
    let mut check = Check::new(
        "a_phpstan_error_is_reported_with_its_rule_place_fix_and_version",
        "rule php-argument-type (phpstan, level 5), phpstan.neon analyses src; src/example.php line 3 is \
         `takeInt('wrong');`, src/clean.php passes a string to a string; PATH is phpstan's and php's folders then \
         /usr/bin:/bin",
    );
    let repo = php_repo();
    let args = ["check", "--json"];
    let a = cctao_with_path(&repo, &args, Some(&cctao_path_for(&PROGRAMS)));
    check.saw_cctao(&args, a.code, &a.json);
    if let Some(why) = absent_engine_reason(&a, &PROGRAMS, RULE) {
        check.not_run(why);
        return;
    }
    assert_eq!(a.code, 1, "a quality error is the refusal (S-15): {:#}\n{}", a.json, a.stderr);
    let f = a.findings();
    assert_eq!(f.len(), 1, "one finding, in src/example.php; src/clean.php passes: {:#}", a.json);
    assert_eq!(f[0]["rule"], RULE);
    assert_eq!(f[0]["engine"], "phpstan");
    assert_eq!(f[0]["path"], "src/example.php");
    assert_eq!(f[0]["line"], 3);
    assert_eq!(f[0]["fix"], FIX);
    let version = probed_version("phpstan", &["--version"], repo.path());
    assert_eq!(a.json["engines"], json!([{ "engine": "phpstan", "version": version }]), "{:#}", a.json);
}

#[test]
fn phpstan_off_path_is_not_available() {
    let mut check = Check::new(
        "phpstan_off_path_is_not_available",
        "rule php-argument-type (phpstan, level 5) over the same files; cctao runs with PATH /usr/bin:/bin, which holds \
         no phpstan",
    );
    let repo = php_repo();
    let args = ["check", "--json"];
    let a = cctao_with_path(&repo, &args, Some("/usr/bin:/bin"));
    check.saw_cctao(&args, a.code, &a.json);
    assert_eq!(a.code, 3, "not available has its own code: {:#}\n{}", a.json, a.stderr);
    assert!(a.findings().is_empty(), "nothing is reported as found: {:#}", a.json);
    assert_eq!(not_available_pairs(&a), vec![(RULE.to_string(), "phpstan".to_string())], "{:#}", a.json);
}

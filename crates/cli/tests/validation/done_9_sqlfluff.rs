//! Done 9 (sqlfluff): a rule naming the engine reports a known violation, in a file the
//! test made, with its rule, its place and its fix text, and the report lists the engine's version; with the engine
//! off the PATH, the same rule reports "not available" (exit 3). One rule, `engine = "sqlfluff"`, `check = "LT01"`
//! (spacing); .sqlfluff sets the ansi dialect (cctao passes none); example.sql has two spaces before FROM on line 2,
//! clean.sql one. cctao runs with PATH the folder that gives it sqlfluff (`cctao_path_for`) then /usr/bin:/bin. Only
//! when sqlfluff is not on the PATH `cctao` gets (`on_cctao_path`, which runs nothing) is the reported case NOT RUN,
//! with `cctao`'s reason, never PASS; `cctao` must then report the rule not available.

use serde_json::json;

use crate::absent_engine_reason::absent_engine_reason;
use crate::check::Check;
use crate::not_available_pairs::not_available_pairs;
use crate::probed_version::probed_version;
use crate::repo::Repo;
use crate::cctao::cctao_with_path;
use crate::cctao_path_for::cctao_path_for;

const RULE: &str = "sql-spacing";
const FIX: &str = "Use a single space.";

const RULES: &str = r#"version = 1

[[rule]]
id = "sql-spacing"
engine = "sqlfluff"
check = "LT01"
severity = "error"
fix = "Use a single space."
"#;

const DIALECT: &str = "[sqlfluff]\ndialect = ansi\n";

// Line 2 of EXAMPLE has two spaces before FROM; CLEAN has one everywhere.
const EXAMPLE: &str = "SELECT a FROM t;\nSELECT b  FROM t;\n";
const CLEAN: &str = "SELECT a FROM t;\n";

/// A repository holding the rule file, .sqlfluff, example.sql and clean.sql, committed.
fn sql_repo() -> Repo {
    let repo = Repo::new();
    repo.write("tao.toml", RULES);
    repo.write(".sqlfluff", DIALECT);
    repo.write("example.sql", EXAMPLE);
    repo.write("clean.sql", CLEAN);
    repo.commit("sql files");
    repo
}

#[test]
fn a_sqlfluff_violation_is_reported_with_its_rule_place_fix_and_version() {
    let mut check = Check::new(
        "a_sqlfluff_violation_is_reported_with_its_rule_place_fix_and_version",
        "rule sql-spacing (sqlfluff, LT01), dialect ansi in .sqlfluff; example.sql line 2 is `SELECT b  FROM t;`, \
         clean.sql is single-spaced; PATH is sqlfluff's folder then /usr/bin:/bin",
    );
    let repo = sql_repo();
    let args = ["check", "--json"];
    let a = cctao_with_path(&repo, &args, Some(&cctao_path_for(&["sqlfluff"])));
    check.saw_cctao(&args, a.code, &a.json);
    if let Some(why) = absent_engine_reason(&a, &["sqlfluff"], RULE) {
        check.not_run(why);
        return;
    }
    assert_eq!(a.code, 1, "a quality error is the refusal: {:#}\n{}", a.json, a.stderr);
    let f = a.findings();
    assert_eq!(f.len(), 1, "one finding, in example.sql; clean.sql passes: {:#}", a.json);
    assert_eq!(f[0]["rule"], RULE);
    assert_eq!(f[0]["engine"], "sqlfluff");
    assert_eq!(f[0]["path"], "example.sql");
    assert_eq!(f[0]["line"], 2);
    assert_eq!(f[0]["fix"], FIX);
    let version = probed_version("sqlfluff", &["--version"], repo.path());
    assert_eq!(a.json["engines"], json!([{ "engine": "sqlfluff", "version": version }]), "{:#}", a.json);
}

#[test]
fn sqlfluff_off_path_is_not_available() {
    let mut check = Check::new(
        "sqlfluff_off_path_is_not_available",
        "rule sql-spacing (sqlfluff, LT01) over the same files; cctao runs with PATH /usr/bin:/bin, which holds no \
         sqlfluff",
    );
    let repo = sql_repo();
    let args = ["check", "--json"];
    let a = cctao_with_path(&repo, &args, Some("/usr/bin:/bin"));
    check.saw_cctao(&args, a.code, &a.json);
    assert_eq!(a.code, 3, "not available has its own code: {:#}\n{}", a.json, a.stderr);
    assert!(a.findings().is_empty(), "nothing is reported as found: {:#}", a.json);
    assert_eq!(not_available_pairs(&a), vec![(RULE.to_string(), "sqlfluff".to_string())], "{:#}", a.json);
}

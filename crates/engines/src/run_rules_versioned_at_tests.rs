//! ZUT for run_rules_versioned_at.rs, a compatibility check only: on rules that run nothing and read no base (a tessera
//! rule, never available, and a pylint rule, with no adapter) in a folder that does not exist, any base gives
//! `run_rules_versioned`'s check. No unit test can see the base arrive through `run_rules_versioned_at`: a rule that
//! reads it needs a resolved folder, and that runs git. The ZIT crates/engines/tests/change_counts.rs reads it both
//! ways, through `check_change` with and without the base, so no copy is written here. The pieces are proved by
//! check_host_tests.rs (the base goes into the check's host) and git/run_git_with_tests.rs (the git entry reads it).

use crate::{run_rules_versioned, run_rules_versioned_at, CheckedRules, EngineVersion, RuleRun};
use rules::{Engine, RuleFile, RuleOutcome};
use std::path::Path;

const RULES: &str = r#"version = 1
owner = "repo"
[[rule]]
id = "slop"
engine = "tessera"
check = "slop"
fix = "f"
[[rule]]
id = "lint"
engine = "pylint"
check = "C0114"
fix = "g"
"#;

#[test]
fn any_base_gives_run_rules_versioneds_check_on_rules_that_do_not_read_the_base() {
    let file = RuleFile::parse(RULES, Path::new("engines-zut-no-such-folder")).unwrap();
    let repo = Path::new("/engines-zut-run-rules-versioned-at");
    let not_available = |rule: &str, engine, why: &str| RuleRun {
        rule: rule.to_string(),
        engine,
        outcome: RuleOutcome::NotAvailable { rule: rule.to_string(), engine, why: why.to_string() },
        raw_chars: None,
        no_files: None,
    };
    let want = CheckedRules {
        runs: vec![
            not_available("slop", Engine::Tessera, "Tessera's slop records are not defined yet"),
            not_available("lint", Engine::Pylint, "no adapter for pylint in this build"),
        ],
        engines: vec![
            EngineVersion { engine: Engine::Tessera, version: None },
            EngineVersion { engine: Engine::Pylint, version: None },
        ],
    };
    assert_eq!(run_rules_versioned(&file, repo, &[]), want);
    for base in [None, Some("abc123")] {
        assert_eq!(run_rules_versioned_at(&file, repo, &[], base), want, "base {base:?}");
    }
}

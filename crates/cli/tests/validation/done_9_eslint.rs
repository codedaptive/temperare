//! Done 9 (eslint) (SPEC-engines-common Done 9): a rule naming the engine reports a known violation, in a file the
//! test made, with its rule, its place and its fix text, and the report lists the engine's version; with the engine
//! off the PATH, the same rule reports "not available" (exit 3). One rule, `engine = "eslint"`, `check = "no-var"`,
//! over the repository's own eslint.config.mjs (`export default [{}];`, no rule of its own); example.js declares
//! with `var` on line 2, clean.js with `const`. cctao runs with PATH the folders that give it eslint and node
//! (`cctao_path_for`) then /usr/bin:/bin: eslint is a `#!/usr/bin/env node` script, and the entry needs both eslint and
//! node, so cctao looks for each on that PATH. Only when eslint is not on the PATH
//! `cctao` gets (`on_cctao_path`, which runs nothing) is the reported case NOT RUN, with `cctao`'s reason, never
//! PASS; `cctao` must then report the rule not available. eslint found in the repository's node_modules/.bin is
//! not covered here; tests/world/local_eslint_real.rs covers it.

use serde_json::json;

use crate::absent_engine_reason::absent_engine_reason;
use crate::check::Check;
use crate::not_available_pairs::not_available_pairs;
use crate::probed_version::probed_version;
use crate::repo::Repo;
use crate::cctao::cctao_with_path;
use crate::cctao_path_for::cctao_path_for;

const RULE: &str = "js-no-var";
const FIX: &str = "Declare with `let` or `const`.";

const RULES: &str = r#"version = 1

[[rule]]
id = "js-no-var"
engine = "eslint"
check = "no-var"
severity = "error"
fix = "Declare with `let` or `const`."
"#;

// An empty config warns on stderr; `[{}]` is ESLint's own advice for a config with nothing in it.
const CONFIG: &str = "export default [{}];\n";
// Line 2 of EXAMPLE declares with `var`; CLEAN uses `const` only.
const EXAMPLE: &str = "const a = 1;\nvar x = a;\nconsole.log(x);\n";
const CLEAN: &str = "const y = 2;\nconsole.log(y);\n";

// git is in /usr/bin; eslint is in neither folder (it is installed elsewhere, such as /opt/homebrew/bin).
const PATH_WITHOUT_ENGINES: &str = "/usr/bin:/bin";

/// A repository holding the rule file, eslint.config.mjs, example.js and clean.js, committed.
fn js_repo() -> Repo {
    let repo = Repo::new();
    repo.write("tao.toml", RULES);
    repo.write("eslint.config.mjs", CONFIG);
    repo.write("example.js", EXAMPLE);
    repo.write("clean.js", CLEAN);
    repo.commit("javascript files");
    repo
}

#[test]
fn an_eslint_problem_is_reported_with_its_rule_place_fix_and_version() {
    let mut check = Check::new(
        "an_eslint_problem_is_reported_with_its_rule_place_fix_and_version",
        "rule js-no-var (eslint, no-var); eslint.config.mjs `export default [{}];`; example.js line 2 is `var x = a;`, \
         clean.js uses `const`; PATH is eslint's and node's folders then /usr/bin:/bin",
    );
    let repo = js_repo();
    let args = ["check", "--json"];
    let path = cctao_path_for(&["eslint", "node"]);
    let a = cctao_with_path(&repo, &args, Some(&path));
    check.saw_cctao(&args, a.code, &a.json);
    if let Some(why) = absent_engine_reason(&a, &["eslint"], RULE) {
        check.not_run(why);
        return;
    }
    assert_eq!(a.code, 1, "a quality error is the refusal (S-15): {:#}\n{}", a.json, a.stderr);
    let f = a.findings();
    assert_eq!(f.len(), 1, "one finding, in example.js; clean.js passes: {:#}", a.json);
    assert_eq!(f[0]["rule"], RULE);
    assert_eq!(f[0]["engine"], "eslint");
    assert_eq!(f[0]["path"], "example.js");
    assert_eq!(f[0]["line"], 2);
    assert_eq!(f[0]["fix"], FIX);
    let version = probed_version("eslint", &["--version"], repo.path());
    assert_eq!(a.json["engines"], json!([{ "engine": "eslint", "version": version }]), "{:#}", a.json);
}

#[test]
fn eslint_off_path_is_not_available() {
    let mut check = Check::new(
        "eslint_off_path_is_not_available",
        "rule js-no-var (eslint, no-var) over the same files; cctao runs with PATH /usr/bin:/bin, which holds no eslint",
    );
    let repo = js_repo();
    let args = ["check", "--json"];
    let a = cctao_with_path(&repo, &args, Some(PATH_WITHOUT_ENGINES));
    check.saw_cctao(&args, a.code, &a.json);
    assert_eq!(a.code, 3, "not available has its own code: {:#}\n{}", a.json, a.stderr);
    assert!(a.findings().is_empty(), "nothing is reported as found: {:#}", a.json);
    assert_eq!(not_available_pairs(&a), vec![(RULE.to_string(), "eslint".to_string())], "{:#}", a.json);
}

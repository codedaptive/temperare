//! `write_head_once`: the head lines of the `make accept` report that only the checks can see (S-31): the built
//! `tm`'s version, then each engine as that `tm` sees it, from `tm rules --json` over one rule per engine: the engine's
//! version when `tm` says it is available, else "not available" with `tm`'s reason. Written once per run, to
//! `00-head.md` in the parts folder; `scripts/accept.sh` puts the commit, the date and the test run's exit code above
//! them.

use std::ffi::OsString;
use std::fs;
use std::path::Path;
use std::sync::Once;
use std::time::Duration;

use procio::{run, Cmd, ProcError};
use serde_json::Value;

use crate::clean_cmd::clean_cmd;
use crate::env::clean_env;
use crate::repo::Repo;
use crate::run_clean::run_clean;

static HEAD: Once = Once::new();

/// One rule per engine, only so `tm rules` says whether each engine is available; nothing is checked with them.
const RULES: &str = r#"version = 1

[[rule]]
id = "head-clippy"
engine = "clippy"
check = "clippy::bool_comparison"
fix = "none"

[[rule]]
id = "head-swiftlint"
engine = "swiftlint"
check = "force_cast"
fix = "none"

[[rule]]
id = "head-ruff"
engine = "ruff"
check = "E711"
fix = "none"

[[rule]]
id = "head-ast-grep"
engine = "ast-grep"
check = "rules/head.yml"
fix = "none"

[[rule]]
id = "head-tessera"
engine = "tessera"
check = "erosion"
fix = "none"
"#;

/// For each of `tm`'s five engines, by its rule-file name: the program it runs as (engines' `program`) and the
/// arguments that print its version. Tessera's is asked only once `tm` says Tessera is available, which it does not
/// yet (S-20).
const VERSIONS: [(&str, &str, &[&str]); 5] = [
    ("clippy", "cargo", &["clippy", "--version"]),
    ("swiftlint", "swiftlint", &["version"]),
    ("ruff", "ruff", &["--version"]),
    ("ast-grep", "ast-grep", &["--version"]),
    ("tessera", "ts", &["--version"]),
];

/// Writes the head part into `parts` the first time it is called in this run; later calls do nothing.
pub fn write_head_once(parts: &Path) {
    HEAD.call_once(|| {
        let mut lines = vec![format!("- tm: {}", version(env!("CARGO_BIN_EXE_temperare"), &["--version"]))];
        lines.extend(engine_lines());
        fs::write(parts.join("00-head.md"), lines.join("\n") + "\n").expect("write the report head");
    });
}

/// One line per rule of `tm rules --json` over RULES, run in a throwaway repository in the environment `tm` gets; one
/// line saying why when its answer holds no rules list.
fn engine_lines() -> Vec<String> {
    let repo = Repo::new();
    repo.write("temperare.toml", RULES);
    let out = run_clean(env!("CARGO_BIN_EXE_temperare"), &["rules", "--json"], repo.path(), clean_env(None));
    let listing: Value = match serde_json::from_slice(&out.stdout) {
        Ok(listing) => listing,
        Err(e) => return vec![format!("- engines: not recorded; tm rules --json printed no JSON ({e})")],
    };
    let Some(rules) = listing["rules"].as_array() else {
        return vec![format!("- engines: not recorded; tm rules --json listed no rules: {listing}")];
    };
    rules.iter().map(engine_line).collect()
}

/// `- <engine>: <version>` when `tm` says the engine is available, else `- <engine>: not available (tm: <why>)`.
fn engine_line(rule: &Value) -> String {
    let engine = rule["engine"].as_str().unwrap_or("?");
    if rule["available"] != true {
        return format!("- {engine}: not available (tm: {})", rule["why"].as_str().unwrap_or("no reason given"));
    }
    let (_, program, args) = VERSIONS
        .iter()
        .find(|(name, ..)| *name == engine)
        .unwrap_or_else(|| panic!("tm listed engine {engine}, which is not one of its five"));
    format!("- {engine}: {}", version(program, args))
}

/// The first line `program args` prints (stdout, else stderr), run in the temp folder and the environment `tm`
/// gets; "no version printed" when it prints nothing; "not installed" when procio finds no such program; procio's
/// error when it gives no answer otherwise; the exit code when the program fails.
fn version(program: &str, args: &[&str]) -> String {
    let cmd = Cmd {
        timeout: Some(Duration::from_secs(120)),
        ..clean_cmd(OsString::from(program), args, &std::env::temp_dir(), clean_env(None))
    };
    match run(&cmd) {
        Err(ProcError::NotFound { .. }) => "not installed".to_string(),
        Err(e) => format!("no answer: {e}"),
        Ok(out) if out.code == Some(0) => {
            let printed = if out.stdout.is_empty() { &out.stderr } else { &out.stdout };
            let first = String::from_utf8_lossy(printed).lines().next().unwrap_or("").trim().to_string();
            if first.is_empty() {
                "no version printed".to_string()
            } else {
                first
            }
        }
        Ok(out) => format!("`{program} {}` failed with exit {:?}", args.join(" "), out.code),
    }
}

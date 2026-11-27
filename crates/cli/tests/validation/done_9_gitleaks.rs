//! Acceptance (gitleaks): a rule naming the engine reports a known violation, in a file the
//! test made, with its rule, its place and its fix text, and the report lists the engine's version; with the engine
//! off the PATH, the same rule reports "not available" (exit 3). One security rule, `engine = "gitleaks"`,
//! `check = "aws-access-token"`, under a top-level owner; config.py holds an AWS access key on line 1. The key is
//! built at run time (the engines test's key), so it never sits whole in this source, and no output `cctao`
//! gives, `--json` or text, holds it. cctao runs with PATH the folder that gives it gitleaks (`cctao_path_folder`) then
//! /usr/bin:/bin. Only when gitleaks is not on the PATH `cctao` gets (`on_cctao_path`, which runs nothing) is the
//! reported case NOT RUN, with `cctao`'s reason, never PASS; `cctao` must then report the rule not available.

use serde_json::json;

use crate::absent_engine_reason::absent_engine_reason;
use crate::check::Check;
use crate::env::cctao_env;
use crate::probed_version::probed_version;
use crate::repo::Repo;
use crate::run_clean::run_clean;
use crate::temp_home::TempHome;
use crate::cctao::cctao_with_path;
use crate::cctao_path_folder::cctao_path_folder;

const RULE: &str = "no-aws-key";
const FIX: &str = "Remove the key and rotate it.";

// git is in /usr/bin; gitleaks is in neither folder (it is installed elsewhere, such as /opt/homebrew/bin).
const PATH_WITHOUT_ENGINES: &str = "/usr/bin:/bin";

/// The fake AWS access key gitleaks' aws-access-token rule matches, in two parts so the source never holds it whole.
fn key() -> String {
    format!("AKIA{}", "Q7Z2M4X6W3B6N5KD")
}

/// The rule file: one security rule of gitleaks' aws-access-token, owned by the file's top-level owner.
fn rules(fix: &str) -> String {
    format!(
        "version = 1\nowner = \"platform\"\n\n[[rule]]\nid = \"{RULE}\"\nengine = \"gitleaks\"\n\
         check = \"aws-access-token\"\nkind = \"security\"\nseverity = \"error\"\nfix = \"{fix}\"\n"
    )
}

/// A repository holding the rule file and config.py, whose line 1 assigns the key, committed.
fn secret_repo() -> Repo {
    let repo = Repo::new();
    repo.write("tao.toml", &rules(FIX));
    repo.write("config.py", &format!("aws_key = \"{}\"\n", key()));
    repo.commit("a leaked key");
    repo
}

#[test]
fn a_gitleaks_secret_is_reported_with_its_rule_place_fix_and_version() {
    let mut check = Check::new(
        "a_gitleaks_secret_is_reported_with_its_rule_place_fix_and_version",
        "rule no-aws-key (gitleaks, aws-access-token, kind security), rule file owner platform; config.py line 1 \
         assigns an AWS access key; PATH is gitleaks' folder then /usr/bin:/bin; run with --json, then as text",
    );
    let key = key();
    let masked = |text: &str| text.replace(&key, "[key]");
    let repo = secret_repo();
    let args = ["check", "--json"];
    let path = cctao_path_folder("gitleaks")
        .map_or(PATH_WITHOUT_ENGINES.to_string(), |dir| format!("{dir}:{PATH_WITHOUT_ENGINES}"));
    let a = cctao_with_path(&repo, &args, Some(&path));
    check.saw_cctao(&args, a.code, &a.json);
    if let Some(why) = absent_engine_reason(&a, &["gitleaks"], RULE) {
        check.not_run(masked(&why));
        return;
    }
    let shown = masked(&format!("{:#}\n{}", a.json, a.stderr));
    assert_eq!(a.code, 4, "a security finding never refuses: {shown}");
    // The leak checks come before the field checks, whose own messages would print a leaked value.
    assert!(!a.json.to_string().contains(&key), "the key is in --json's stdout: {shown}");
    assert!(!a.stderr.contains(&key), "the key is in --json's stderr: {shown}");
    let f = a.findings();
    assert_eq!(f.len(), 1, "one finding, in config.py: {shown}");
    assert_eq!(f[0]["rule"], RULE);
    assert_eq!(f[0]["engine"], "gitleaks");
    assert_eq!(f[0]["path"], "config.py");
    assert_eq!(f[0]["line"], 1);
    assert_eq!(f[0]["fix"], FIX);
    let version = probed_version("gitleaks", &["version"], repo.path());
    assert_eq!(a.json["engines"], json!([{ "engine": "gitleaks", "version": version }]), "{shown}");

    let home = TempHome::new();
    let out = run_clean(env!("CARGO_BIN_EXE_cctao"), &["check"], repo.path(), cctao_env(Some(&path), home.path()));
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    let stderr = String::from_utf8_lossy(&out.stderr).to_string();
    let shown = masked(&format!("{stdout}\n{stderr}"));
    assert_eq!(out.code, Some(4), "the text run exits as the --json run: {shown}");
    assert!(!stdout.contains(&key), "the key is in the text's stdout: {shown}");
    assert!(!stderr.contains(&key), "the key is in the text's stderr: {shown}");
}

#[test]
fn gitleaks_off_path_is_not_available() {
    let mut check = Check::new(
        "gitleaks_off_path_is_not_available",
        "rule no-aws-key (gitleaks, aws-access-token, kind security) over the same files; cctao runs with PATH \
         /usr/bin:/bin, which holds no gitleaks",
    );
    let repo = secret_repo();
    let args = ["check", "--json"];
    let a = cctao_with_path(&repo, &args, Some(PATH_WITHOUT_ENGINES));
    check.saw_cctao(&args, a.code, &a.json);
    assert_eq!(a.code, 3, "not available has its own code: {:#}\n{}", a.json, a.stderr);
    assert!(a.findings().is_empty(), "nothing is reported as found: {:#}", a.json);
    let missing: Vec<(&str, &str)> = a.json["not_available"]
        .as_array()
        .unwrap_or_else(|| panic!("no not_available list: {:#}", a.json))
        .iter()
        .map(|n| (n["rule"].as_str().unwrap_or(""), n["engine"].as_str().unwrap_or("")))
        .collect();
    assert_eq!(missing, vec![(RULE, "gitleaks")], "{:#}", a.json);
}

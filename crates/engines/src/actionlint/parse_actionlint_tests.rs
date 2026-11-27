//! Unit test for parse_actionlint.rs on real actionlint 1.7.12 output (fixtures/fixtures.txt records each capture). The
//! captures' paths are relative to the run's folder, so `DIR` stands for it as `cwd`, and as `folder` in every row but
//! the folder-holding-none row; no folder is read.

use super::parse_actionlint;
use crate::types::{Hit, ParseError};
use procio::Output;
use rules::RelPath;
use std::path::Path;

const DIR: &str = "/w/repo";

fn hit(path: &str, line: Option<u32>, col: Option<u32>, what: &str) -> Hit {
    Hit { path: RelPath::new(path), line, col, what: what.to_string() }
}

#[test]
fn parse_actionlint_cases() {
    let ci = ".github/workflows/ci.yml";
    let release = ".github/workflows/release.yml";
    let expression = vec![
        hit(ci, Some(10), Some(24), "undefined variable \"unknown\". available variables are \"env\", \"github\", \"inputs\", \"job\", \"matrix\", \"needs\", \"runner\", \"secrets\", \"steps\", \"strategy\", \"vars\""),
        hit(release, Some(10), Some(24), "property \"nope\" is not defined in object type {action: string; action_path: string; action_ref: string; action_repository: string; action_status: string; actor: string; actor_id: string; api_url: string; artifact_cache_size_limit: number; base_ref: string; env: string; event: object; event_name: string; event_path: string; graphql_url: string; head_ref: string; job: string; output: string; path: string; ref: string; ref_name: string; ref_protected: bool; ref_type: string; repository: string; repository_id: string; repository_owner: string; repository_owner_id: string; repository_visibility: string; repositoryurl: string; retention_days: number; run_attempt: string; run_id: string; run_number: string; secret_source: string; server_url: string; sha: string; state: string; step_summary: string; token: string; triggering_actor: string; workflow: string; workflow_ref: string; workflow_sha: string; workspace: string}"),
    ];
    let action = vec![
        hit(ci, Some(9), Some(11), "input \"nonexistent-input\" is not defined in action \"actions/checkout@v4\". available inputs are \"clean\", \"fetch-depth\", \"fetch-tags\", \"filter\", \"github-server-url\", \"lfs\", \"path\", \"persist-credentials\", \"ref\", \"repository\", \"set-safe-directory\", \"show-progress\", \"sparse-checkout\", \"sparse-checkout-cone-mode\", \"ssh-key\", \"ssh-known-hosts\", \"ssh-strict\", \"ssh-user\", \"submodules\", \"token\""),
        hit(release, Some(9), Some(11), "input \"node-versio\" is not defined in action \"actions/setup-node@v4\". available inputs are \"always-auth\", \"architecture\", \"cache\", \"cache-dependency-path\", \"check-latest\", \"mirror\", \"mirror-token\", \"node-version\", \"node-version-file\", \"registry-url\", \"scope\", \"token\""),
    ];
    let shellcheck = vec![hit(".github/workflows/shell.yml", Some(7), Some(9),
        "shellcheck reported issue in this script: SC2086:info:1:6: Double quote to prevent globbing and word splitting")];
    // Built by hand: an error with no line or column, and one in a file outside the folder.
    let no_place = br#"{"kinds":[],"errors":[{"message":"Outside.","filepath":"/elsewhere/x.yml","line":1,"column":1,"kind":"syntax-check"},{"message":"No place.","filepath":"./.github/workflows/a.yml","line":0,"column":0,"kind":"syntax-check"}]}"#;
    let config_error = b"could not parse config file \"bad-config.yaml\": yaml: line 1: did not find expected ',' or ']'\n";
    let findings: &[u8] = include_bytes!("fixtures/findings.json");
    let rows: [(&str, &[u8], &[u8], &str, &str, Result<Vec<Hit>, ParseError>); 9] = [
        ("expression", findings, b"", "expression", DIR, Ok(expression)),
        ("action", findings, b"", "action", DIR, Ok(action)),
        ("a kind with no error", findings, b"", "shellcheck", DIR, Ok(vec![])),
        ("folder holding none", findings, b"", "expression", "/w/repo/sub", Ok(vec![])),
        ("clean", include_bytes!("fixtures/clean.json"), b"", "expression", DIR, Ok(vec![])),
        ("shellcheck", include_bytes!("fixtures/shellcheck.json"), b"", "shellcheck", DIR, Ok(shellcheck)),
        ("no line or column, file outside", no_place, b"", "syntax-check", DIR,
            Ok(vec![hit(".github/workflows/a.yml", None, None, "No place.")])),
        ("config it cannot parse, stderr quoted", b"", config_error, "expression", DIR, Err(ParseError {
            why: "actionlint printed no JSON report: could not parse config file \"bad-config.yaml\": yaml: line 1: did not find expected ',' or ']'\n".to_string(),
        })),
        ("not the report, stdout quoted", b"[]", b"", "expression", DIR,
            Err(ParseError { why: "actionlint printed no JSON report: []".to_string() })),
    ];
    for (label, stdout, stderr, check, folder, expected) in rows {
        let out = Output { code: Some(1), stdout: stdout.to_vec(), stderr: stderr.to_vec() };
        assert_eq!(parse_actionlint(&out, check, Path::new(DIR), Path::new(folder)), expected, "{label}");
    }
}

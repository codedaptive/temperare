//! ZUT for run_gitleaks.rs: `run_gitleaks(&GITLEAKS, ..)` on a recording host: the environment removed from the
//! command, the outcome read from captured gitleaks 8.30.1 output (fixtures/, see fixtures.txt) and literal stderr,
//! and the raw count left in `raw_chars`. Nothing runs.

use super::{run_gitleaks, GITLEAKS};
use crate::host::Host;
use crate::types::{Hit, Stop};
use procio::{Cmd, Output};
use rules::{RelPath, RuleFile};
use std::cell::{Cell, RefCell};
use std::ffi::OsString;
use std::path::Path;

const DIR: &str = "/w/leak";

#[test]
fn run_gitleaks_keeps_the_config_cuts_stderr_and_counts_raw() {
    let key = format!("AKIA{}", "Q7Z2M4X6W3B6N5KD");
    let found = "10:04PM INF Overriding enabled rules: aws-access-token\n10:04PM INF scanned ~33 bytes (33 bytes) in 54.6ms\n10:04PM WRN leaks found: 1\n";
    let failing = format!(
        "10:04PM INF Overriding enabled rules: aws-access-token\nFinding:     aws_key = \"{key}\"\nSecret:      {key}\nRuleID:      aws-access-token\n10:04PM FTL Failed to load config error=\"open lint/missing.toml: no such file or directory\"\n"
    );
    let hit = Hit {
        path: RelPath::new("config.py"),
        line: Some(1),
        col: Some(12),
        what: "gitleaks found a secret here; tm does not print it".to_string(),
    };
    let rows: [(&str, &str, Output, Result<Vec<Hit>, Stop>, usize); 3] = [
        (
            "a finding, INF and WRN lines on stderr",
            "aws-access-token",
            Output { code: Some(1), stdout: include_bytes!("fixtures/aws_access_token.sarif").to_vec(), stderr: found.into() },
            Ok(vec![hit]),
            49800,
        ),
        (
            "an FTL line after a verbose finding block",
            "aws-access-token",
            Output { code: Some(1), stdout: Vec::new(), stderr: failing.into_bytes() },
            Err(Stop::Failed {
                code: Some(1),
                stderr: "Failed to load config error=\"open lint/missing.toml: no such file or directory\"".to_string(),
            }),
            257,
        ),
        (
            "an unknown rule id",
            "no-such-rule",
            Output { code: Some(1), stdout: Vec::new(), stderr: include_bytes!("fixtures/unknown_rule.txt").to_vec() },
            Err(Stop::UnknownCheck { said: "Requested rule no-such-rule not found in rules".to_string() }),
            110,
        ),
    ];
    for (case, check, answer, want, raw) in rows {
        let text = format!("version = 1\n[[rule]]\nid = \"aws\"\nengine = \"gitleaks\"\ncheck = \"{check}\"\nfix = \"f\"\n");
        let file = RuleFile::parse(&text, Path::new(DIR)).unwrap();
        let seen = RefCell::new(None);
        let run = |cmd: &Cmd| {
            *seen.borrow_mut() = Some(cmd.clone());
            Ok(answer.clone())
        };
        let on_path = |_: &str| -> bool { panic!("gitleaks' run looks nothing up on PATH") };
        let raw_chars = Cell::new(None);
        let host = Host { run: &run, on_path: &on_path, raw_chars: &raw_chars, base: None };
        let got = run_gitleaks(&GITLEAKS, &file.rules[0], Path::new(DIR), &host);
        assert!(!format!("{got:?}").contains(&key), "{case}: the key reached the outcome");
        assert_eq!(got, want, "{case}");
        assert_eq!(raw_chars.get(), Some(raw), "{case}");
        let removed = seen.into_inner().unwrap().env_remove;
        assert_eq!(removed, [OsString::from("GITLEAKS_CONFIG"), OsString::from("GITLEAKS_CONFIG_TOML")], "{case}");
    }
}

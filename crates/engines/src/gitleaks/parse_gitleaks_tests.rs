//! Unit test for parse_gitleaks.rs: real gitleaks 8.30.1 SARIF logs (fixtures/, see fixtures.txt) and logs holding a fake
//! key built at runtime, read into hits that carry cctao's fixed text. `DIR` is the folder gitleaks ran in; a narrower
//! rule folder tests an unplaced result.

use super::parse_gitleaks;
use crate::types::{Hit, ParseError};
use procio::Output;
use rules::RelPath;
use std::path::Path;

const DIR: &str = "/w/leak";
const WHAT: &str = "gitleaks found a secret here; cctao does not print it";
const LEAKY_LOG: &str = r#"{"runs":[{"results":[{"ruleId":"aws-access-token","message":{"text":"found KEY"},"locations":[{"physicalLocation":{"artifactLocation":{"uri":"config.py"},"region":{"startLine":2,"startColumn":5,"snippet":{"text":"KEY"}}}}]}]}]}"#;

fn hit(line: u32, col: u32) -> Hit {
    Hit { path: RelPath::new("config.py"), line: Some(line), col: Some(col), what: WHAT.to_string() }
}

#[test]
fn parse_gitleaks_reads_hits_without_gitleaks_text() {
    let key = format!("AKIA{}", "Q7Z2M4X6W3B6N5KD");
    let no_log = ParseError { why: "gitleaks printed no SARIF log and no error".to_string() };
    let rows: [(&str, Vec<u8>, &str, Result<Vec<Hit>, ParseError>); 6] = [
        ("the aws fixture", include_bytes!("fixtures/aws_access_token.sarif").to_vec(), DIR, Ok(vec![hit(1, 12)])),
        ("the clean fixture", include_bytes!("fixtures/clean.sarif").to_vec(), DIR, Ok(vec![])),
        ("a log whose message and snippet hold the key", LEAKY_LOG.replace("KEY", &key).into_bytes(), DIR, Ok(vec![hit(2, 5)])),
        ("a log of another rule", LEAKY_LOG.replace("KEY", &key).replace("aws-access-token", "jwt").into_bytes(), DIR, Ok(vec![])),
        ("stdout that is no SARIF log", format!("Secret:      {key}\n").into_bytes(), DIR, Err(no_log)),
        ("a result outside the rule folder", LEAKY_LOG.replace("KEY", &key).into_bytes(), "/w/leak/sub", Err(ParseError {
            why: "SARIF result names `config.py`, a path not under the rule folder `/w/leak/sub`".to_string(),
        })),
    ];
    for (name, stdout, folder, want) in rows {
        let out = Output { code: Some(1), stdout, stderr: Vec::new() };
        let got = parse_gitleaks(&out, "aws-access-token", Path::new(DIR), Path::new(folder));
        assert!(!format!("{got:?}").contains(&key), "{name}: the key reached the result");
        assert_eq!(got, want, "{name}");
    }
}

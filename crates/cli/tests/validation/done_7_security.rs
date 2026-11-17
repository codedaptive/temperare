//! Done 7: a security rule (mission must match 9). "A finding of a rule of kind `security` is reported with its
//! severity, location and owner, and the run does not end with the refusing exit code. The same rule run alone over
//! the tree reports the same finding." Security-only exits 4, never 1 (S-15); the owner is the file's (S-9, S-12).

use crate::repo::Repo;
use crate::tm::tm;

const RULES: &str = r#"version = 1
owner = "payments"

[[rule]]
id = "no-hardcoded-password"
engine = "ruff"
check = "S105"
kind = "security"
severity = "error"
fix = "Read the secret from the environment or the keychain."
"#;

#[test]
fn a_security_finding_is_reported_with_severity_place_and_owner_and_never_refuses() {
    let repo = Repo::new();
    repo.write("temperare.toml", RULES);
    repo.write("settings.py", "password = \"hunter2\"\n");
    repo.commit("settings");

    let a = tm(&repo, &["check", "--json"]);
    assert_eq!(a.code, 4, "security alone never refuses (S-15): {:#}\n{}", a.json, a.stderr);
    let f = a.findings();
    assert_eq!(f.len(), 1, "one finding: {:#}", a.json);
    assert_eq!(f[0]["rule"], "no-hardcoded-password");
    assert_eq!(f[0]["kind"], "security");
    assert_eq!(f[0]["severity"], "error");
    assert_eq!(f[0]["owner"], "payments");
    assert_eq!(f[0]["path"], "settings.py");
    assert_eq!(f[0]["line"], 1);

    let alone = tm(&repo, &["check", "--json", "--kind", "security"]);
    assert_eq!(alone.code, 4, "the same exit alone: {:#}", alone.json);
    assert_eq!(alone.findings(), a.findings(), "the security rules alone report the same finding");
}

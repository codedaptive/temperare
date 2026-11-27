//! Done 7: a security rule (mission must match 9). "A finding of a rule of kind `security` is reported with its
//! severity, location and owner, and the run does not end with the refusing exit code. The same rule run alone over
//! the tree reports the same finding." Security-only exits 4, never 1 (S-15); the owner is the file's (S-9, S-12).

use crate::check::Check;
use crate::repo::Repo;
use crate::cctao::cctao;

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
    let mut check = Check::new(
        "a_security_finding_is_reported_with_severity_place_and_owner_and_never_refuses",
        "rule no-hardcoded-password (ruff, S105, kind security), rule file owner payments; settings.py line 1 holds \
         a hard-coded password; run over the tree, then with --kind security",
    );
    let repo = Repo::new();
    repo.write("tao.toml", RULES);
    repo.write("settings.py", "password = \"hunter2\"\n");
    repo.commit("settings");

    let args = ["check", "--json"];
    let a = cctao(&repo, &args);
    check.saw_cctao(&args, a.code, &a.json);
    assert_eq!(a.code, 4, "security alone never refuses (S-15): {:#}\n{}", a.json, a.stderr);
    let f = a.findings();
    assert_eq!(f.len(), 1, "one finding: {:#}", a.json);
    assert_eq!(f[0]["rule"], "no-hardcoded-password");
    assert_eq!(f[0]["kind"], "security");
    assert_eq!(f[0]["severity"], "error");
    assert_eq!(f[0]["owner"], "payments");
    assert_eq!(f[0]["path"], "settings.py");
    assert_eq!(f[0]["line"], 1);

    let alone_args = ["check", "--json", "--kind", "security"];
    let alone = cctao(&repo, &alone_args);
    check.saw_cctao(&alone_args, alone.code, &alone.json);
    assert_eq!(alone.code, 4, "the same exit alone: {:#}", alone.json);
    assert_eq!(alone.findings(), a.findings(), "the security rules alone report the same finding");
}

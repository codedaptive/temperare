//! ZUT for gitleaks_refusal.rs: which checks are refused and the reason given, written out from the contract.

use super::gitleaks_refusal;

#[test]
fn gitleaks_refusal_refuses_an_empty_part_and_all() {
    let rows: [(&str, &str, Option<&str>); 7] = [
        ("a built-in id", "aws-access-token", None),
        ("a comma list", "aws-access-token,generic-api-key", None),
        ("a repo's own id", "Our_Token", None),
        ("empty", "", Some("gitleaks check ``: an empty part is not a gitleaks rule id")),
        ("a trailing comma", "aws-access-token,", Some("gitleaks check `aws-access-token,`: an empty part is not a gitleaks rule id")),
        ("ALL", "ALL", Some("gitleaks check `ALL`: `ALL` is not a gitleaks rule id")),
        ("ALL in a list", "aws-access-token,ALL", Some("gitleaks check `aws-access-token,ALL`: `ALL` is not a gitleaks rule id")),
    ];
    for (name, check, want) in rows {
        assert_eq!(gitleaks_refusal(check), want.map(str::to_string), "{name}");
    }
}

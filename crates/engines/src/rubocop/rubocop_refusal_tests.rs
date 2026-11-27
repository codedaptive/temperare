//! Unit test for rubocop_refusal.rs: which checks are refused and the reason given, the cases written out from the contract.

use super::rubocop_refusal;

#[test]
fn rubocop_refusal_refuses_any_part_that_is_not_a_full_cop_name() {
    let cases: [(&str, Option<&str>); 12] = [
        ("Lint/UselessAssignment", None),
        ("Style/StringLiterals,Lint/Debugger", None),
        ("RSpec/Capybara/CurrentPathExpectation", None),
        ("Naming/VariableNumber2", None),
        ("Style", Some("rubocop check `Style`: `Style` is not a full cop name such as `Lint/UselessAssignment`")),
        ("", Some("rubocop check ``: `` is not a full cop name such as `Lint/UselessAssignment`")),
        ("Lint/UselessAssignment,", Some("rubocop check `Lint/UselessAssignment,`: `` is not a full cop name such as `Lint/UselessAssignment`")),
        ("Lint/UselessAssignment, Style/StringLiterals", Some("rubocop check `Lint/UselessAssignment, Style/StringLiterals`: ` Style/StringLiterals` is not a full cop name such as `Lint/UselessAssignment`")),
        ("-a", Some("rubocop check `-a`: `-a` is not a full cop name such as `Lint/UselessAssignment`")),
        ("Style/*", Some("rubocop check `Style/*`: `Style/*` is not a full cop name such as `Lint/UselessAssignment`")),
        ("lint/UselessAssignment", Some("rubocop check `lint/UselessAssignment`: `lint/UselessAssignment` is not a full cop name such as `Lint/UselessAssignment`")),
        ("Lint/", Some("rubocop check `Lint/`: `Lint/` is not a full cop name such as `Lint/UselessAssignment`")),
    ];
    let mut failed = Vec::new();
    for (check, want) in cases {
        let result = std::panic::catch_unwind(|| assert_eq!(rubocop_refusal(check).as_deref(), want, "{check}"));
        if result.is_err() { failed.push(check); }
    }
    assert!(failed.is_empty(), "failed rows: {failed:?}");
}

use crate::{test_json::to_json, RuleState};

#[test]
fn rule_states_serialize_as_snake_case() {
    for (state, json) in [
        (RuleState::Ran, "\"ran\""),
        (RuleState::NotAvailable, "\"not_available\""),
        (RuleState::Failed, "\"failed\""),
        (RuleState::NotApplicable, "\"not_applicable\""),
        (RuleState::CoversNothing, "\"covers_nothing\""),
    ] {
        assert_eq!(to_json(&state), json, "{state:?}");
    }
}

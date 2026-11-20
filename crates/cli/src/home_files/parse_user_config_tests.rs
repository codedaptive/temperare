//! ZUT for parse_user_config: switch defaults, valid values and one-line errors.

use super::parse_user_config;

#[test]
fn savings_contract_accepts_boolean_switch_values() {
    assert_eq!(parse_user_config("savings_line = false"), Ok(false));
    assert_eq!(parse_user_config("savings_line = true"), Ok(true));
}

#[test]
fn savings_contract_defaults_empty_and_comment_only_config_to_true() {
    assert_eq!(parse_user_config(""), Ok(true));
    assert_eq!(parse_user_config("# c\n"), Ok(true));
}

#[test]
fn savings_contract_rejects_non_boolean_switch() {
    assert_eq!(parse_user_config("savings_line = 0"), Err("savings_line must be true or false".into()));
}

#[test]
fn savings_contract_rejects_unknown_keys_even_with_a_valid_switch() {
    assert_eq!(parse_user_config("colour = 1"), Err("unknown key `colour`".into()));
    assert_eq!(parse_user_config("savings_line = false\nx = 1"), Err("unknown key `x`".into()));
}

#[test]
fn savings_contract_selects_first_unknown_key_in_sorted_order() {
    assert_eq!(parse_user_config("z = 1\na = 2"), Err("unknown key `a`".into()));
}

#[test]
fn savings_contract_unknown_key_holding_a_newline_is_escaped_to_one_line() {
    let error = parse_user_config("\"a\\nb\" = 1").unwrap_err();
    assert_eq!(error, "unknown key `a\\nb`");
    assert!(!error.contains('\n'));
}

#[test]
fn savings_contract_invalid_toml_gives_a_single_line_error() {
    let error = parse_user_config("savings_line = ").unwrap_err();
    assert!(!error.is_empty());
    assert!(!error.contains('\n'));
}

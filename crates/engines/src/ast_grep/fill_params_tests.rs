//! ZUT for fill_params.rs: string and list params, the regex escape, unknown and bad params, a param the YAML never
//! names, unclosed braces.

use serde_json::json;

use super::fill_params;
use crate::ParamError;

#[test]
fn a_string_param_fills_as_written() {
    let got = fill_params("message: use {{ api }} here\n", &json!({"api": "a.b(c)"}));
    assert_eq!(got, Ok("message: use a.b(c) here\n".to_string()));
}

#[test]
fn a_list_param_is_escaped_and_joined_by_bars() {
    let got = fill_params("regex: '^(?:{{names}})$'", &json!({"names": ["a.b", "c|d", "e"]}));
    assert_eq!(got, Ok(r"regex: '^(?:a\.b|c\|d|e)$'".to_string()));
}

#[test]
fn an_empty_list_fills_with_nothing() {
    assert_eq!(fill_params("x{{names}}y", &json!({"names": []})), Ok("xy".to_string()));
}

#[test]
fn every_document_of_a_multi_document_rule_is_filled() {
    let yaml = "id: one\nregex: {{names}}\n---\nid: two\nregex: {{names}}\nnote: {{why}}\n";
    let got = fill_params(yaml, &json!({"names": ["x+"], "why": "because"}));
    assert_eq!(got, Ok("id: one\nregex: x\\+\n---\nid: two\nregex: x\\+\nnote: because\n".to_string()));
}

#[test]
fn a_name_not_in_params_is_unknown() {
    let got = fill_params("a {{ missing }} b", &json!({"names": ["x"]}));
    assert_eq!(got, Err(ParamError::Unknown { name: "missing".to_string() }));
}

#[test]
fn a_number_param_is_bad() {
    let got = fill_params("n: {{count}}", &json!({"count": 3}));
    assert_eq!(
        got,
        Err(ParamError::Bad { name: "count".to_string(), why: "not a string or a list of strings".to_string() })
    );
}

#[test]
fn a_table_param_is_bad() {
    let got = fill_params("{{t}}", &json!({"t": {"k": "v"}}));
    assert!(matches!(got, Err(ParamError::Bad { name, .. }) if name == "t"));
}

#[test]
fn a_list_with_a_non_string_item_is_bad() {
    let got = fill_params("{{names}}", &json!({"names": ["a", 2]}));
    assert_eq!(got, Err(ParamError::Bad { name: "names".to_string(), why: "a list item is not a string".to_string() }));
}

#[test]
fn params_that_are_not_an_object_are_bad() {
    let got = fill_params("{{names}}", &json!(["a"]));
    assert!(matches!(got, Err(ParamError::Bad { name, .. }) if name == "params"));
}

#[test]
fn an_unclosed_open_brace_pair_stays_as_written() {
    let got = fill_params("a {{x}} b {{ c", &json!({"x": "1"}));
    assert_eq!(got, Ok("a 1 b {{ c".to_string()));
}

#[test]
fn text_with_no_braces_comes_back_unchanged() {
    let yaml = "id: plain\nlanguage: Rust\nrule:\n  pattern: foo($A)\n";
    assert_eq!(fill_params(yaml, &json!({})), Ok(yaml.to_string()));
}

#[test]
fn a_param_the_yaml_never_names_is_bad_listing_its_placeholders_in_order_once() {
    let got = fill_params("b: {{ beta }}\na: {{alpha}}\nb2: {{beta}}\n", &json!({"alpha": "1", "beta": "2", "gamma": "3"}));
    let why = "the rule YAML has no {{gamma}}; its placeholders are {{beta}}, {{alpha}}";
    assert_eq!(got, Err(ParamError::Bad { name: "gamma".to_string(), why: why.to_string() }));
}

#[test]
fn a_param_given_to_a_yaml_with_no_placeholders_is_bad() {
    let got = fill_params("id: plain\nrule:\n  pattern: foo($A)\n", &json!({"names": ["x"]}));
    let why = "the rule YAML has no {{names}} and no placeholders";
    assert_eq!(got, Err(ParamError::Bad { name: "names".to_string(), why: why.to_string() }));
}

#[test]
fn an_unknown_placeholder_is_reported_before_an_unused_param() {
    let got = fill_params("a {{missing}} b", &json!({"extra": "1"}));
    assert_eq!(got, Err(ParamError::Unknown { name: "missing".to_string() }));
}

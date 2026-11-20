//! ZUT for temp_config.rs, the paths that write no file: a kind other than `Config`, params that are not a table,
//! and params the config's text cannot use. The written file is the ZIT's (integration/temp_config_tests.rs).

use super::entry_cmd_tests::fake_entry;
use super::{temp_config, ConfigParams, Params, Pick};
use crate::ParamError;
use serde_json::json;

fn refusing_width() -> Params {
    Params::Config(ConfigParams {
        option: "--config=",
        ext: "toml",
        text: |_| Err(ParamError::Bad { name: "width".to_string(), why: "must be a number".to_string() }),
    })
}

#[test]
fn a_kind_other_than_config_writes_nothing() {
    for params in [Params::Args(&[]), Params::Template] {
        let got = temp_config(&fake_entry(Pick::Arg("--select="), params, None), &json!({"width": 3}));
        assert!(matches!(got, Ok(None)), "{params:?}");
    }
}

#[test]
fn params_the_text_cannot_use_give_the_param_why() {
    let got = temp_config(&fake_entry(Pick::Arg("--select="), refusing_width(), None), &json!({"width": "x"}));
    assert_eq!(got.err(), Some("lint param width: must be a number".to_string()));
}

#[test]
fn params_that_are_not_a_table_give_the_param_why() {
    let got = temp_config(&fake_entry(Pick::Arg("--select="), refusing_width(), None), &json!(["width"]));
    assert_eq!(got.err(), Some("lint param params: not a table of named params".to_string()));
}

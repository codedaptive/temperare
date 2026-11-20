//! ZUT for threshold_of.rs: a threshold in range is read with its number; 0, past the max, a string, a float, the
//! wrong check and a second threshold are `Bad`; a dashed check matches an underscored threshold check; no threshold
//! given, and an entry without thresholds, give `None`.

use super::entry_cmd_tests::fake_entry;
use super::{threshold_of, Entry, ParamArg, Params, Pick, Reach, Threshold};
use crate::ParamError;
use serde_json::{json, Value};

static ALLOW: [ParamArg; 1] = [ParamArg { name: "allow", flag: "--allow=" }];
static LIMITS: [Threshold; 2] = [
    Threshold { name: "max_complexity", check: "C901", max: 50, setting: |n| format!("c={n}") },
    Threshold { name: "max_nesting", check: "nesting_depth", max: 9, setting: |n| format!("n={n}") },
];

fn limited() -> Entry {
    let params = Params::Thresholds { args: &ALLOW, thresholds: &LIMITS, reach: Reach::Arg("--config=") };
    fake_entry(Pick::Arg("--select="), params, None)
}

/// The threshold's name and number, which is what a caller reads from it.
fn of(entry: &Entry, check: &str, params: Value) -> Result<Option<(&'static str, u32)>, ParamError> {
    threshold_of(entry, check, &params).map(|found| found.map(|(threshold, n)| (threshold.name, n)))
}

fn bad(name: &str, why: &str) -> Result<Option<(&'static str, u32)>, ParamError> {
    Err(ParamError::Bad { name: name.to_string(), why: why.to_string() })
}

#[test]
fn a_threshold_in_range_is_read_with_its_number() {
    assert_eq!(of(&limited(), "C901", json!({"max_complexity": 12})), Ok(Some(("max_complexity", 12))));
    assert_eq!(of(&limited(), "C901", json!({"max_complexity": 1})), Ok(Some(("max_complexity", 1))));
    assert_eq!(of(&limited(), "C901", json!({"max_complexity": 50})), Ok(Some(("max_complexity", 50))));
}

#[test]
fn the_setting_is_given_the_number() {
    let (threshold, n) = threshold_of(&limited(), "C901", &json!({"max_complexity": 7})).unwrap().unwrap();
    assert_eq!((threshold.setting)(n), "c=7");
    assert_eq!((threshold.check, threshold.max), ("C901", 50));
}

#[test]
fn zero_and_past_the_max_are_bad() {
    let why = "must be a whole number from 1 to 50";
    assert_eq!(of(&limited(), "C901", json!({"max_complexity": 0})), bad("max_complexity", why));
    assert_eq!(of(&limited(), "C901", json!({"max_complexity": 51})), bad("max_complexity", why));
    assert_eq!(of(&limited(), "C901", json!({"max_complexity": -3})), bad("max_complexity", why));
}

#[test]
fn a_string_or_a_float_is_bad() {
    let why = "must be a whole number from 1 to 50";
    assert_eq!(of(&limited(), "C901", json!({"max_complexity": "5"})), bad("max_complexity", why));
    assert_eq!(of(&limited(), "C901", json!({"max_complexity": 5.5})), bad("max_complexity", why));
}

#[test]
fn a_threshold_for_another_check_is_bad() {
    let got = of(&limited(), "E711", json!({"max_complexity": 5}));
    assert_eq!(got, bad("max_complexity", "read only with check `C901`"));
}

#[test]
fn a_dashed_check_matches_an_underscored_threshold_check() {
    assert_eq!(of(&limited(), "nesting-depth", json!({"max_nesting": 3})), Ok(Some(("max_nesting", 3))));
    assert_eq!(of(&limited(), "nesting_depth", json!({"max_nesting": 3})), Ok(Some(("max_nesting", 3))));
}

#[test]
fn a_second_threshold_is_bad() {
    let got = of(&limited(), "C901", json!({"max_complexity": 5, "max_nesting": 3}));
    assert_eq!(got, bad("max_nesting", "only one threshold per rule"));
}

#[test]
fn no_threshold_given_is_none() {
    assert_eq!(of(&limited(), "C901", json!({})), Ok(None));
    assert_eq!(of(&limited(), "C901", json!({"allow": ["x"]})), Ok(None));
}

#[test]
fn params_that_are_not_a_table_are_bad() {
    assert_eq!(of(&limited(), "C901", json!([1])), bad("params", "not a table of named params"));
}

#[test]
fn an_entry_without_thresholds_is_none_whatever_the_params() {
    let entry = fake_entry(Pick::Arg("--select="), Params::Args(&ALLOW), None);
    assert_eq!(of(&entry, "C901", json!({"max_complexity": 0})), Ok(None));
}

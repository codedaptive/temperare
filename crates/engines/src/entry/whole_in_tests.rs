//! Unit tests for whole_in.rs: a whole number from `min` to `max` comes back; anything else, a missing key included, is
//! `Bad` named after the param with the range in its why.

use super::whole_in;
use crate::ParamError;
use serde_json::{json, Map, Value};

fn table(value: Value) -> Map<String, Value> {
    let Value::Object(map) = json!({ "floor": value }) else { unreachable!() };
    map
}

#[test]
fn whole_in_reads_a_whole_number_in_range_and_refuses_the_rest() {
    let why = "must be a whole number from 2 to 9";
    let out_of_range = || Err(ParamError::Bad { name: "floor".into(), why: why.into() });
    let rows: [(&str, Value, Result<u32, ParamError>); 8] = [
        ("in range", json!(5), Ok(5)),
        ("min", json!(2), Ok(2)),
        ("max", json!(9), Ok(9)),
        ("min - 1", json!(1), out_of_range()),
        ("max + 1", json!(10), out_of_range()),
        ("negative", json!(-3), out_of_range()),
        ("float", json!(2.5), out_of_range()),
        ("string", json!("3"), out_of_range()),
    ];
    for (case, value, want) in rows {
        assert_eq!(whole_in(&table(value), "floor", 2, 9), want, "{case}");
    }
}

#[test]
fn whole_in_refuses_a_missing_key_the_same_way() {
    let want = Err(ParamError::Bad { name: "max_ratio".into(), why: "must be a whole number from 0 to 100".into() });
    assert_eq!(whole_in(&table(json!(4)), "max_ratio", 0, 100), want);
}

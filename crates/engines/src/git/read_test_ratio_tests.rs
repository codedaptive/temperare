//! Unit tests for read_test_ratio.rs.

use super::read_test_ratio;
use crate::ParamError;
use rules::RelPath;
use serde_json::{json, Map, Value};

fn params(value: Value) -> Map<String, Value> {
    let Value::Object(map) = value else { panic!("not an object") };
    map
}

fn bad(name: &str, why: &str) -> ParamError {
    ParamError::Bad { name: name.to_string(), why: why.to_string() }
}

#[test]
fn good_params_are_read() {
    let cases = [
        (json!({"max_ratio": 1.5, "floor": 40, "test_paths": ["**/tests/**"]}), 1.5, 40),
        (json!({"max_ratio": 2, "floor": 0, "test_paths": ["**/tests/**"]}), 2.0, 0),
        (json!({"max_ratio": 0.1, "floor": 100000, "test_paths": ["**/tests/**"]}), 0.1, 100000),
        (json!({"max_ratio": 100, "floor": 7, "test_paths": ["**/tests/**"], "other": 1}), 100.0, 7),
    ];
    for (given, max_ratio, floor) in cases {
        let ratio = read_test_ratio(&params(given.clone())).unwrap_or_else(|e| panic!("{given}: {e:?}"));
        assert_eq!((ratio.max_ratio, ratio.floor), (max_ratio, floor), "{given}");
        assert!(ratio.test_paths.matches(&RelPath::new("a/tests/b.rs")), "{given}");
        assert!(!ratio.test_paths.matches(&RelPath::new("a/src/b.rs")), "{given}");
    }
}

#[test]
fn a_missing_or_wrong_param_is_bad() {
    let needs = "a git rule needs it";
    let number = "must be a number from 0.1 to 100";
    let whole = "must be a whole number from 0 to 100000";
    let globs = "must be a list of glob strings";
    let cases = [
        (json!({"floor": 40, "test_paths": ["t/**"]}), bad("max_ratio", needs)),
        (json!({"max_ratio": 1.5, "test_paths": ["t/**"]}), bad("floor", needs)),
        (json!({"max_ratio": 1.5, "floor": 40}), bad("test_paths", needs)),
        (json!({}), bad("max_ratio", needs)),
        (json!({"max_ratio": 0.05, "floor": 40, "test_paths": ["t/**"]}), bad("max_ratio", number)),
        (json!({"max_ratio": 100.5, "floor": 40, "test_paths": ["t/**"]}), bad("max_ratio", number)),
        (json!({"max_ratio": "1.5", "floor": 40, "test_paths": ["t/**"]}), bad("max_ratio", number)),
        (json!({"max_ratio": 1.5, "floor": -1, "test_paths": ["t/**"]}), bad("floor", whole)),
        (json!({"max_ratio": 1.5, "floor": 100001, "test_paths": ["t/**"]}), bad("floor", whole)),
        (json!({"max_ratio": 1.5, "floor": 2.5, "test_paths": ["t/**"]}), bad("floor", whole)),
        (json!({"max_ratio": 1.5, "floor": 40, "test_paths": []}), bad("test_paths", globs)),
        (json!({"max_ratio": 1.5, "floor": 40, "test_paths": [1]}), bad("test_paths", globs)),
        (json!({"max_ratio": 1.5, "floor": 40, "test_paths": "x"}), bad("test_paths", globs)),
    ];
    for (given, want) in cases {
        assert_eq!(read_test_ratio(&params(given.clone())).err(), Some(want), "{given}");
    }
}

#[test]
fn a_bad_glob_is_bad_with_its_text() {
    let given = params(json!({"max_ratio": 1.5, "floor": 40, "test_paths": ["a["]}));
    let Err(ParamError::Bad { name, why }) = read_test_ratio(&given) else { panic!("a[ was read") };
    assert_eq!(name, "test_paths");
    assert!(why.starts_with("bad glob: bad glob \"a[\""), "{why}");
}

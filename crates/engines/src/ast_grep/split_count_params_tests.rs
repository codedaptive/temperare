//! Unit tests for split_count_params.rs: each refusal's text in the order the checks run, count mode with and without
//! shared_setup, and the params left for the fill.

use serde_json::{json, Map, Value};

use super::split_count_params;
use rules::RelPath;

fn object(value: Value) -> Map<String, Value> {
    match value {
        Value::Object(map) => map,
        _ => panic!("not an object"),
    }
}

#[test]
fn bad_params_are_refused_with_the_first_check_that_fails() {
    let rows = [
        (json!("names"), "param params in r.yml: not a table of named params"),
        (json!({"count": "lines"}), "param count in r.yml: takes only \"setup-vs-tests\""),
        (json!({"count": 3}), "param count in r.yml: takes only \"setup-vs-tests\""),
        (json!({"count": "lines", "shared_setup": 3}), "param count in r.yml: takes only \"setup-vs-tests\""),
        (json!({"shared_setup": ["a/**"]}), "param shared_setup in r.yml: needs count = \"setup-vs-tests\""),
        (json!({"shared_setup": 3}), "param shared_setup in r.yml: needs count = \"setup-vs-tests\""),
        (json!({"count": "setup-vs-tests", "shared_setup": "a/**"}), "param shared_setup in r.yml: not a list of globs"),
        (json!({"count": "setup-vs-tests", "shared_setup": ["a", 1]}), "param shared_setup in r.yml: not a list of globs"),
    ];
    for (params, want) in rows {
        let got = split_count_params(&params, "r.yml").map(|(mode, rest)| (mode.is_some(), rest));
        assert_eq!(got, Err(want.to_string()), "{params}");
    }
}

/// The bad glob is named in rules' own words; the reason after it is globset's and is not pinned.
#[test]
fn a_bad_glob_is_refused_naming_the_glob() {
    let params = json!({"count": "setup-vs-tests", "shared_setup": ["a/**", "a["]});
    let Err(got) = split_count_params(&params, "r.yml") else { panic!("a[ was accepted") };
    assert!(got.starts_with("param shared_setup in r.yml: bad glob \"a[\": "), "{got}");
}

#[test]
fn count_with_shared_setup_exempts_only_its_globs() {
    let params = json!({"count": "setup-vs-tests", "shared_setup": ["tests/support/**", "*.fixture.rs"]});
    let (mode, rest) = split_count_params(&params, "r.yml").unwrap();
    let exempt = mode.unwrap().shared_setup.unwrap();
    let rows = [
        ("tests/support/world.rs", true),
        ("tests/support/deep/x.rs", true),
        ("big.fixture.rs", true),
        ("src/big.fixture.rs", false),
        ("tests/x_tests.rs", false),
    ];
    for (path, want) in rows {
        assert_eq!(exempt.matches(&RelPath::new(path)), want, "{path}");
    }
    assert_eq!(rest, Map::new());
}

#[test]
fn count_without_shared_setup_or_with_an_empty_list_exempts_nothing() {
    for params in [json!({"count": "setup-vs-tests"}), json!({"count": "setup-vs-tests", "shared_setup": []})] {
        let (mode, rest) = split_count_params(&params, "r.yml").unwrap();
        assert!(mode.unwrap().shared_setup.is_none(), "{params}");
        assert_eq!(rest, Map::new(), "{params}");
    }
}

#[test]
fn no_count_and_no_shared_setup_is_no_count_mode_and_the_params_unchanged() {
    let params = json!({"names": ["a", "b"], "why": "because"});
    let (mode, rest) = split_count_params(&params, "r.yml").unwrap();
    assert!(mode.is_none());
    assert_eq!(rest, object(json!({"names": ["a", "b"], "why": "because"})));
}

#[test]
fn count_mode_keeps_the_other_params_for_the_fill() {
    let params = json!({"count": "setup-vs-tests", "shared_setup": ["a/**"], "names": ["x"]});
    let (mode, rest) = split_count_params(&params, "r.yml").unwrap();
    assert!(mode.is_some());
    assert_eq!(rest, object(json!({"names": ["x"]})));
}

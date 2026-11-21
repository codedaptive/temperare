//! Unit tests for stop.rs: plain data; equality covers the variant and every field, `UnknownCheck` included.

use super::Stop;

fn failed(code: Option<i32>, stderr: &str) -> Stop {
    Stop::Failed { code, stderr: stderr.to_string() }
}

#[test]
fn stops_compare_by_variant_and_every_field() {
    let absent = Stop::NotAvailable { why: "ruff not found on PATH".to_string() };
    assert_eq!(absent, Stop::NotAvailable { why: "ruff not found on PATH".to_string() });
    assert_ne!(absent, Stop::NotAvailable { why: "swiftlint not found on PATH".to_string() });
    assert_eq!(failed(Some(2), "bad"), failed(Some(2), "bad"));
    assert_ne!(failed(Some(2), "bad"), failed(None, "bad"));
    assert_ne!(failed(Some(2), "bad"), failed(Some(2), "worse"));
    assert_ne!(absent, failed(None, "ruff not found on PATH"));
}

#[test]
fn an_unknown_check_compares_by_what_the_engine_said_and_is_no_other_stop() {
    let unknown = |said: &str| Stop::UnknownCheck { said: said.to_string() };
    assert_eq!(unknown("Unknown rule selector: `ZZZ999`"), unknown("Unknown rule selector: `ZZZ999`"));
    assert_ne!(unknown("Unknown rule selector: `ZZZ999`"), unknown("Unknown rule selector: `E711`"));
    assert_ne!(unknown("x"), Stop::NotAvailable { why: "x".to_string() });
    assert_ne!(unknown("x"), failed(None, "x"));
}

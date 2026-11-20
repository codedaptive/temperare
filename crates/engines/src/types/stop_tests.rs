//! Unit tests for stop.rs: plain data; equality covers the variant and every field.

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

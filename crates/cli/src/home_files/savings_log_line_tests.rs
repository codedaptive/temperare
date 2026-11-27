//! Unit test for savings_log_line: exact key order and values, and JSON escaping (the UTC dates: utc_rfc3339_tests).

use super::savings_log_line;
use std::path::Path;
use serde_json::json;

#[test]
fn savings_contract_epoch_line_has_exact_keys_order_and_values() {
    assert_eq!(savings_log_line(0, Path::new("/work/repo"), 12, 34),
        "{\"time\":\"1970-01-01T00:00:00Z\",\"folder\":\"/work/repo\",\"verb\":\"check\",\"output_tokens\":12,\"baseline_tokens\":34}");
}

#[test]
fn savings_contract_escapes_folder_quotes_and_preserves_json_values() {
    let line = savings_log_line(0, Path::new("/work/a\"b"), 12, 34);
    assert_eq!(line, "{\"time\":\"1970-01-01T00:00:00Z\",\"folder\":\"/work/a\\\"b\",\"verb\":\"check\",\"output_tokens\":12,\"baseline_tokens\":34}");
    assert_eq!(serde_json::from_str::<serde_json::Value>(&line).unwrap(), json!({
        "time":"1970-01-01T00:00:00Z", "folder":"/work/a\"b", "verb":"check", "output_tokens":12, "baseline_tokens":34
    }));
}

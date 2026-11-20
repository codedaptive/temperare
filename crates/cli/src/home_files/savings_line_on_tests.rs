//! Unit tests for savings_line_on that need no disk: an absent or empty HOME returns before any file is read.

use super::savings_line_on;
use std::path::Path;

#[test]
fn savings_contract_absent_or_empty_home_defaults_silently() {
    let mut stderr = Vec::new();
    assert!(savings_line_on(None, &mut stderr));
    assert!(savings_line_on(Some(Path::new("")), &mut stderr));
    assert!(stderr.is_empty());
}

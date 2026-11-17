//! ZUT for write_usage_error.rs: the two lines tm prints for a usage error.

use super::write_usage_error::write_usage_error;
use crate::UsageError;

#[test]
fn writes_what_then_next_each_on_its_own_line() {
    let mut err = Vec::new();
    let e = UsageError { what: "--rules given twice".to_string(), fix: "Give --rules once.".to_string() };
    write_usage_error(&mut err, &e);
    assert_eq!(String::from_utf8(err).unwrap(), "tm: --rules given twice\nnext: Give --rules once.\n");
}

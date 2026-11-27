//! ZUT for failed.rs: cctao's own failure text kept whole, with no exit code.

use crate::failed::failed;
use crate::types::Stop;

#[test]
fn the_text_is_the_failure_with_no_exit_code() {
    let got = failed("cannot read x: denied".to_string());
    assert_eq!(got, Stop::Failed { code: None, stderr: "cannot read x: denied".to_string() });
}

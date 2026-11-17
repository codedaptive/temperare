//! ZUT for param_error.rs: plain data; the two cases and their fields tell errors apart.

use super::ParamError;

#[test]
fn param_errors_compare_by_case_and_fields() {
    let unknown = ParamError::Unknown { name: "names".to_string() };
    let bad = ParamError::Bad { name: "names".to_string(), why: "a number".to_string() };
    assert_eq!(unknown.clone(), ParamError::Unknown { name: "names".to_string() });
    assert_ne!(unknown, ParamError::Unknown { name: "other".to_string() });
    assert_ne!(bad, ParamError::Bad { name: "names".to_string(), why: "a table".to_string() });
    assert_ne!(unknown, bad);
}

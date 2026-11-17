use crate::Output;

fn ok() -> Output {
    Output { code: Some(0), stdout: b"out".to_vec(), stderr: b"err".to_vec() }
}

#[test]
fn a_clone_is_equal() {
    assert_eq!(ok().clone(), ok());
}

#[test]
fn killed_by_a_signal_differs_from_exit_zero() {
    assert_ne!(Output { code: None, ..ok() }, ok());
}

#[test]
fn each_stream_takes_part_in_equality() {
    assert_ne!(Output { stdout: vec![], ..ok() }, ok());
    assert_ne!(Output { stderr: vec![], ..ok() }, ok());
}

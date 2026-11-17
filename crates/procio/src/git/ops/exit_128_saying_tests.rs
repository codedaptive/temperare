use super::exit_128_saying::exit_128_saying;
use crate::Output;

fn out(code: Option<i32>, stderr: &[u8]) -> Output {
    Output { code, stdout: Vec::new(), stderr: stderr.to_vec() }
}

#[test]
fn exit_128_with_the_message_inside_stderr_is_true() {
    let o = out(Some(128), b"fatal: not a git repository (or any of the parent directories): .git\n");
    assert!(exit_128_saying(&o, &[b"not a git repository"]));
}

#[test]
fn any_one_of_several_messages_is_enough() {
    let o = out(Some(128), b"fatal: path 'a' exists on disk, but not in 'HEAD'\n");
    assert!(exit_128_saying(&o, &[b"does not exist in", b"exists on disk, but not in"]));
}

#[test]
fn another_exit_code_with_the_message_is_false() {
    assert!(!exit_128_saying(&out(Some(1), b"not a git repository"), &[b"not a git repository"]));
    assert!(!exit_128_saying(&out(None, b"not a git repository"), &[b"not a git repository"]));
}

#[test]
fn exit_128_with_another_message_is_false() {
    let o = out(Some(128), b"fatal: cannot change to '/x': No such file or directory\n");
    assert!(!exit_128_saying(&o, &[b"not a git repository"]));
}

#[test]
fn empty_stderr_or_no_messages_is_false() {
    assert!(!exit_128_saying(&out(Some(128), b""), &[b"not a git repository"]));
    assert!(!exit_128_saying(&out(Some(128), b"anything"), &[]));
}

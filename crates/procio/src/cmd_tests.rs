use std::path::PathBuf;
use std::time::Duration;

use crate::Cmd;

fn full() -> Cmd {
    Cmd {
        program: "prog".into(),
        args: vec!["a".into()],
        cwd: Some(PathBuf::from("/tmp")),
        env: vec![("K".into(), "V".into())],
        env_remove: vec!["R".into()],
        stdin: b"in".to_vec(),
        timeout: Some(Duration::from_secs(1)),
    }
}

#[test]
fn a_clone_is_equal() {
    assert_eq!(full().clone(), full());
}

#[test]
fn each_field_takes_part_in_equality() {
    let base = full();
    assert_ne!(Cmd { program: "other".into(), ..full() }, base);
    assert_ne!(Cmd { args: vec![], ..full() }, base);
    assert_ne!(Cmd { cwd: None, ..full() }, base);
    assert_ne!(Cmd { env: vec![], ..full() }, base);
    assert_ne!(Cmd { env_remove: vec![], ..full() }, base);
    assert_ne!(Cmd { stdin: vec![], ..full() }, base);
    assert_ne!(Cmd { timeout: None, ..full() }, base);
}

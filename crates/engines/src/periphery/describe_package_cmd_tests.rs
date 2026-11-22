use super::describe_package_cmd;
use procio::Cmd;
use std::path::{Path, PathBuf};

#[test]
fn describe_package_cmd_cases() {
    let expected = Cmd {
        args: ["package", "--scratch-path", "/tmp/build", "describe", "--type", "json"].iter().map(Into::into).collect(),
        cwd: Some(PathBuf::from("/work")),
        ..Cmd::new("swift")
    };
    assert_eq!(describe_package_cmd(Path::new("/work"), Path::new("/tmp/build")), expected);
}

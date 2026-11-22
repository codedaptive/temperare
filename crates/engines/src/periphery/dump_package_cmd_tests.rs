use super::dump_package_cmd;
use procio::Cmd;
use std::path::{Path, PathBuf};

#[test]
fn dump_package_cmd_cases() {
    let expected = Cmd {
        args: ["package", "--scratch-path", "/tmp/build", "dump-package"].iter().map(Into::into).collect(),
        cwd: Some(PathBuf::from("/work")),
        ..Cmd::new("swift")
    };
    assert_eq!(dump_package_cmd(Path::new("/work"), Path::new("/tmp/build")), expected);
}

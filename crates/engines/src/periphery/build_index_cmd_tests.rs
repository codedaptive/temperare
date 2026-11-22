use super::build_index_cmd;
use procio::Cmd;
use std::path::{Path, PathBuf};

#[test]
fn build_index_cmd_cases() {
    let expected = Cmd {
        args: ["build", "--build-tests", "--build-system", "swiftbuild", "--scratch-path", "/tmp/build", "--disable-automatic-resolution"].iter().map(Into::into).collect(),
        cwd: Some(PathBuf::from("/work")),
        ..Cmd::new("swift")
    };
    assert_eq!(build_index_cmd(Path::new("/work"), Path::new("/tmp/build")), expected);
}

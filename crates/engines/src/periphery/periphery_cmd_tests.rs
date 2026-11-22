use super::periphery_cmd;
use procio::Cmd;
use std::path::{Path, PathBuf};

#[test]
fn periphery_cmd_cases() {
    let expected = Cmd {
        args: ["scan", "--skip-build", "--format", "json", "--disable-update-check", "--quiet", "--index-store-path", "/tmp/store", "--json-package-manifest-path", "/tmp/describe.json", "--config", "/tmp/periphery.yml"].iter().map(Into::into).collect(),
        cwd: Some(PathBuf::from("/work")),
        ..Cmd::new("periphery")
    };
    assert_eq!(periphery_cmd(Path::new("/work"), Path::new("/tmp/store"), Path::new("/tmp/describe.json"), Path::new("/tmp/periphery.yml")), expected);
}

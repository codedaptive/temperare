//! `describe_package_cmd`: SwiftPM's description of a package, as a command.

use procio::Cmd;
use std::path::Path;

/// `swift package --scratch-path <build> describe --type json`, run in `root`: the package description Periphery
/// reads through `--json-package-manifest-path` to know the package's targets. `--scratch-path` keeps anything SwiftPM
/// writes in tm's scratch folder rather than the package's `.build`.
pub fn describe_package_cmd(root: &Path, build: &Path) -> Cmd {
    Cmd {
        args: vec![
            "package".into(),
            "--scratch-path".into(),
            build.into(),
            "describe".into(),
            "--type".into(),
            "json".into(),
        ],
        cwd: Some(root.to_path_buf()),
        ..Cmd::new("swift")
    }
}

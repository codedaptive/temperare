//! `dump_package_cmd`: SwiftPM's reading of a package's manifest, as a command.

use procio::Cmd;
use std::path::Path;

/// `swift package --scratch-path <build> dump-package`, run in `root`: the manifest as JSON, which `manifest_walk`
/// reads with `remote_dependencies` and `local_dependencies` before anything is built. It resolves no dependency, so nothing reaches the network
/// (run with the network denied, it exits 0: fixtures/commands.txt), and `--scratch-path` keeps anything SwiftPM writes in tm's scratch folder rather than the package's `.build`.
pub fn dump_package_cmd(root: &Path, build: &Path) -> Cmd {
    Cmd {
        args: vec!["package".into(), "--scratch-path".into(), build.into(), "dump-package".into()],
        cwd: Some(root.to_path_buf()),
        ..Cmd::new("swift")
    }
}

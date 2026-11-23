//! `build_index_cmd`: the build that writes a package's index store for Periphery, as a command.

use procio::Cmd;
use std::path::Path;

/// `swift build --build-tests --build-system swiftbuild --scratch-path <build> --disable-automatic-resolution`, run in
/// `root`. The tests are built too, so code only they use reads as used. swiftbuild writes the index store to
/// `<build>/out`, inside tm's scratch folder, so nothing is built in the package or in DerivedData.
/// `--disable-automatic-resolution` makes a build that would need to resolve a dependency fail instead of fetching it.
pub fn build_index_cmd(root: &Path, build: &Path) -> Cmd {
    Cmd {
        args: vec![
            "build".into(),
            "--build-tests".into(),
            "--build-system".into(),
            "swiftbuild".into(),
            "--scratch-path".into(),
            build.into(),
            "--disable-automatic-resolution".into(),
        ],
        cwd: Some(root.to_path_buf()),
        ..Cmd::new("swift")
    }
}

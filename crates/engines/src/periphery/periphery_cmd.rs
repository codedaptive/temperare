//! `periphery_cmd`: the Periphery scan of a built package, as a command.

use super::PERIPHERY;
use procio::Cmd;
use std::path::Path;

/// `periphery <PERIPHERY.args> --index-store-path <store> --json-package-manifest-path <manifest> --config <config>`,
/// run in `root`. `--skip-build` reads the index store tm's own build wrote; `--config` names the empty file tm
/// wrote, so the package's `.periphery.yml` is never read.
pub fn periphery_cmd(root: &Path, store: &Path, manifest: &Path, config: &Path) -> Cmd {
    let fixed = PERIPHERY.args.iter().map(Into::into);
    let paths = [
        "--index-store-path".into(),
        store.into(),
        "--json-package-manifest-path".into(),
        manifest.into(),
        "--config".into(),
        config.into(),
    ];
    Cmd { args: fixed.chain(paths).collect(), cwd: Some(root.to_path_buf()), ..Cmd::new(PERIPHERY.program) }
}

//! `actionlint_config`: the config file tm passes actionlint, if any.

use std::path::{Path, PathBuf};

/// The configs actionlint itself discovers under the repository's top when given none, in its order (measured on
/// 1.7.12, fixtures/fixtures.txt, "Config discovery": `.yaml` wins when both exist; a sub-folder's own `.github` is
/// not read).
const CONFIG_NAMES: [&str; 2] = [".github/actionlint.yaml", ".github/actionlint.yml"];

/// `folder.join(rule_config)` when the rule names a config, whether or not it exists: a missing one is reported when
/// tm reads it. Otherwise the first of `CONFIG_NAMES` under `top` (the git top) that `exists`; None when none does, and
/// the flow then passes `/dev/null`, so actionlint never discovers a file tm did not read.
pub(crate) fn actionlint_config(
    rule_config: Option<&str>,
    folder: &Path,
    top: &Path,
    exists: &dyn Fn(&Path) -> bool,
) -> Option<PathBuf> {
    if let Some(config) = rule_config {
        return Some(folder.join(config));
    }
    CONFIG_NAMES.iter().map(|name| top.join(name)).find(|path| exists(path))
}

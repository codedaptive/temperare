//! `repo_config`: the repository's own PHPStan config, which the run's wrapper config includes.

use std::path::{Path, PathBuf};

/// The config files PHPStan itself discovers in its working folder when given none, in its order (PHPStan 2.3.7's
/// `findDiscoverableConfigFile`).
const CONFIG_NAMES: [&str; 6] =
    [".phpstan.neon", "phpstan.neon", ".phpstan.neon.dist", "phpstan.neon.dist", ".phpstan.dist.neon", "phpstan.dist.neon"];

/// `folder.join(rule_config)` when the rule names a config, whether or not it exists: a missing one is reported by
/// PHPStan in its own words and the rule fails with them. Otherwise the first of `CONFIG_NAMES` in `folder` that
/// `exists`; None when none does.
pub(crate) fn repo_config(rule_config: Option<&str>, folder: &Path, exists: &dyn Fn(&Path) -> bool) -> Option<PathBuf> {
    if let Some(config) = rule_config {
        return Some(folder.join(config));
    }
    CONFIG_NAMES.iter().map(|name| folder.join(name)).find(|path| exists(path))
}

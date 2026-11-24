//! `yamllint_config`: the repository's own yamllint config, which the config tm hands yamllint extends.

use std::path::{Path, PathBuf};

/// The config files yamllint itself discovers in a folder when given none, in its order (yamllint 1.38.0's
/// `find_project_config_filepath`).
const CONFIG_NAMES: [&str; 3] = [".yamllint", ".yamllint.yaml", ".yamllint.yml"];

/// `folder.join(rule_config)` when the rule names a config, whether or not it exists: the flow checks that and fails
/// the rule naming it, since yamllint given a missing base prints only a traceback. Otherwise the first of
/// `CONFIG_NAMES` that `exists`, in `folder` and then in each ancestor while it is at or below the repository top `top`,
/// nearest folder first, as yamllint walks up from where it runs; it never looks above `top`, so a config outside the
/// checked repository is never the base. None when none exists: the flow then extends yamllint's `default`.
pub(crate) fn yamllint_config(
    rule_config: Option<&str>,
    folder: &Path,
    top: &Path,
    exists: &dyn Fn(&Path) -> bool,
) -> Option<PathBuf> {
    if let Some(config) = rule_config {
        return Some(folder.join(config));
    }
    folder
        .ancestors()
        .take_while(|dir| dir.starts_with(top))
        .flat_map(|dir| CONFIG_NAMES.iter().map(move |name| dir.join(name)))
        .find(|path| exists(path))
}

//! `hadolint_config`: the config file tm pins for a hadolint run in a folder.

use std::path::{Path, PathBuf};

/// The config hadolint is given for a run in `folder`: `folder` joined with `config` when the rule names one; else
/// `folder/.hadolint.yaml` when `exists` says it does, else `folder/.hadolint.yml` when it does; else None. These are
/// the files hadolint 2.15.1 itself reads from its working directory, measured: only that directory, never a parent,
/// and `.hadolint.yaml` before `.hadolint.yml`. Past them hadolint falls back to a machine file (under
/// `$XDG_CONFIG_HOME` or the home folder) that would change findings from one machine to the next; `--config` makes it
/// read the named file and nothing else, so on None the caller passes an empty config in its place.
pub(crate) fn hadolint_config(folder: &Path, config: Option<&str>, exists: &dyn Fn(&Path) -> bool) -> Option<PathBuf> {
    if let Some(config) = config {
        return Some(folder.join(config));
    }
    [".hadolint.yaml", ".hadolint.yml"].into_iter().map(|name| folder.join(name)).find(|path| exists(path))
}

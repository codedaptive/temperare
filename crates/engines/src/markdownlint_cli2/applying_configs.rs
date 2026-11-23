//! `applying_configs`: the markdownlint config files that apply to a rule's files.

use std::path::{Path, PathBuf};

/// The config files markdownlint-cli2 0.24.2 reads in one folder, in its order: it takes the first that exists.
const CONFIG_NAMES: [&str; 6] = [
    ".markdownlint.jsonc",
    ".markdownlint.json",
    ".markdownlint.yaml",
    ".markdownlint.yml",
    ".markdownlint.cjs",
    ".markdownlint.mjs",
];

/// The distinct config files, relative to `folder`, that apply to `files` (relative to `folder`), in the order the
/// files first reach them. For each file, the nearest folder from its own up to `folder` holding a config file that
/// `is_file` finds (given the absolute path), and in it the first of `CONFIG_NAMES`; a file with none in any of those
/// folders adds nothing (markdownlint's defaults, every rule on). As markdownlint-cli2 runs in `folder` (measured: a
/// folder's config replaces its parent's; nothing above the working folder is read).
pub(crate) fn applying_configs(folder: &Path, files: &[PathBuf], is_file: &dyn Fn(&Path) -> bool) -> Vec<PathBuf> {
    let mut configs: Vec<PathBuf> = Vec::new();
    for file in files {
        let nearest = file
            .ancestors()
            .skip(1)
            .find_map(|dir| CONFIG_NAMES.iter().map(|name| dir.join(name)).find(|path| is_file(&folder.join(path))));
        if let Some(config) = nearest.filter(|config| !configs.contains(config)) {
            configs.push(config);
        }
    }
    configs
}

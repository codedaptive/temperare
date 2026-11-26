//! `ValeConfig::read`: the repository's Vale config, from the output of `vale ls-config`.

use super::ValeConfig;
use crate::types::Stop;
use procio::Output;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::path::PathBuf;

/// Why a rule whose folder has no Vale config is not available: it names the five names Vale 3.28.1 looks for, at or
/// above the folder it runs in (fixtures/fixtures.txt, "Config names").
const NO_CONFIG_WHY: &str = "Vale finds no config (.vale.ini, _vale.ini, vale.ini, .vale or _vale) at or above the \
    rule file's folder: tm runs Vale only with the repository's own config";

/// The keys of `vale ls-config`'s JSON that tm reads; Vale 3.28.1 prints `null` for an unset list.
#[derive(Deserialize)]
struct LsConfig {
    #[serde(rename = "SBaseStyles", default)]
    section_styles: Option<BTreeMap<String, Vec<String>>>,
    #[serde(rename = "GBaseStyles", default)]
    global_styles: Option<Vec<String>>,
    #[serde(rename = "Vocab", default)]
    vocab: Option<Vec<String>>,
    #[serde(rename = "Paths", default)]
    paths: Option<Vec<PathBuf>>,
    #[serde(rename = "RootINI")]
    root: PathBuf,
    #[serde(rename = "ConfigFiles", default)]
    files: Option<Vec<PathBuf>>,
}

impl ValeConfig {
    /// `out`, the output of `vale --no-global [--config=<file>] ls-config` run in the rule file's folder, read as the
    /// config Vale resolved there. `styles` holds `GBaseStyles` (the `[*]` section's) then each `SBaseStyles` list in
    /// glob order, each style once; `vocab` and `paths` are `Vocab` and `Paths` as printed; `files` is `RootINI` (when
    /// not empty) then `ConfigFiles`, each once. Vale saying `no config file found` (on stderr, exit 2,
    /// fixtures/no_config.txt) is `NotAvailable` naming the config names Vale reads. Any other output that is not that
    /// JSON (a config Vale cannot load, a `--config` file that does not exist) is `Failed` with Vale's exit code and its
    /// own text: stderr, or stdout when stderr is empty, trimmed.
    pub(crate) fn read(out: &Output) -> Result<ValeConfig, Stop> {
        let stdout = String::from_utf8_lossy(&out.stdout);
        let stderr = String::from_utf8_lossy(&out.stderr);
        if stdout.contains("no config file found") || stderr.contains("no config file found") {
            return Err(Stop::NotAvailable { why: NO_CONFIG_WHY.to_string() });
        }
        let Ok(config) = serde_json::from_str::<LsConfig>(&stdout) else {
            let said = if stderr.trim().is_empty() { stdout } else { stderr };
            return Err(Stop::Failed { code: out.code, stderr: said.trim().to_string() });
        };
        let sections = config.section_styles.into_iter().flat_map(BTreeMap::into_values).flatten();
        let styles = config.global_styles.into_iter().flatten().chain(sections);
        let root = Some(config.root).filter(|root| !root.as_os_str().is_empty());
        Ok(ValeConfig {
            styles: unique(styles),
            vocab: config.vocab.unwrap_or_default(),
            paths: config.paths.unwrap_or_default(),
            files: unique(root.into_iter().chain(config.files.into_iter().flatten())),
        })
    }
}

/// The items of `items` in order, each kept only the first time it comes.
fn unique<T: PartialEq>(items: impl Iterator<Item = T>) -> Vec<T> {
    items.fold(Vec::new(), |mut kept, item| {
        if !kept.contains(&item) {
            kept.push(item);
        }
        kept
    })
}

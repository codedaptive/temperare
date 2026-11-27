//! `config_stop`: what stops a markdownlint rule before its run, from the repository's markdownlint files and
//! cctao's list of rules.

use super::{
    applying_configs, check_off_why, json_settings, md_rule_of, options_file_why, yaml_settings, MD_RULES_FROM,
};
use crate::failed::failed;
use crate::types::Stop;
use std::io;
use std::path::{Path, PathBuf};

/// The stop for a rule with `check` run in `folder` on `files` (the rule's Markdown files, relative to `folder`), or
/// None when it can run. `is_file` and `read` take absolute paths. In order, the first that applies:
/// 1. an options file where markdownlint-cli2 reads one (`options_file_why`) is `Failed` with that why;
/// 2. a check that is no rule name in cctao's list (`md_rule_of`) is `NotListed`, naming the list's source
///    (`MD_RULES_FROM`); `stopped` adds the engine and its version. A tag is not a check;
/// 3. for each config file that applies (`applying_configs`), in turn: a `.jsonc`, `.cjs` or `.mjs` one is `Failed`
///    (a stated limit: cctao has no safe reader for comments-and-commas JSON or a module it would have to run); a file
///    `read` cannot read, or whose text its reader (`json_settings` for `.json`, `yaml_settings` for `.yaml` and
///    `.yml`) refuses, is `Failed` naming the file and saying why; a top-level `extends` key is `Failed` (a stated
///    limit: cctao does not follow it); a config that turns the rule off (`check_off_why`) makes it `NotAvailable` with
///    that why, since markdownlint runs with the repository's own config and would report nothing for it.
pub(crate) fn config_stop(
    check: &str,
    folder: &Path,
    files: &[PathBuf],
    is_file: &dyn Fn(&Path) -> bool,
    read: &dyn Fn(&Path) -> io::Result<String>,
) -> Option<Stop> {
    if let Some(why) = options_file_why(folder, files, is_file) {
        return Some(failed(why));
    }
    let Some(rule) = md_rule_of(check) else {
        let said = format!("{check} is not in cctao's list of markdownlint rules (taken from {MD_RULES_FROM})");
        return Some(Stop::NotListed { said });
    };
    for config in applying_configs(folder, files, is_file) {
        let name = config.display().to_string();
        let reader: fn(&str) -> Result<Vec<(String, bool)>, String> = match config.extension().and_then(|ext| ext.to_str()) {
            Some("json") => json_settings,
            Some("yaml" | "yml") => yaml_settings,
            _ => {
                return Some(failed(format!(
                    "markdownlint config {name}: cctao reads only .markdownlint.json, .yaml and .yml (a stated limit); \
                     give the config in one of those"
                )))
            }
        };
        let text = read(&folder.join(&config)).map_err(|err| err.to_string());
        let settings = match text.and_then(|text| reader(&text)) {
            Ok(settings) => settings,
            Err(err) => return Some(failed(format!("markdownlint config {name} cannot be read: {err}"))),
        };
        if settings.iter().any(|(key, _)| key == "extends") {
            return Some(failed(format!("markdownlint config {name} has `extends`: cctao does not follow it (a stated limit)")));
        }
        if let Some(why) = check_off_why(rule, &name, &settings) {
            return Some(Stop::NotAvailable { why });
        }
    }
    None
}

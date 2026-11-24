//! `config_stop`: why the repository's Vale config cannot serve a check, if it cannot.

use super::ValeConfig;
use crate::types::Stop;
use std::path::Path;

/// `NotAvailable` when `config` cannot serve `check` in the repository whose top is `top` (canonical); None when it
/// can. In order:
/// 1. a config file Vale read that is not under `top`: Vale looks for its config upward past the repository's top,
///    so that is another repository's or a user's config, and tm runs Vale only with the repository's own;
/// 2. the check's style (before the `.`) is in no `BasedOnStyles` of the config: tm never turns on a style the
///    repository did not choose (S-72 (2)). The why names the style and the root config file.
///
/// The config's skip keys and section settings are no stop: the wrapper never copies them (S-67 (1)).
pub(crate) fn config_stop(config: &ValeConfig, check: &str, top: &Path) -> Option<Stop> {
    if let Some(file) = config.files.iter().find(|file| !file.starts_with(top)) {
        return Some(Stop::NotAvailable {
            why: format!(
                "Vale would read the config {}, outside the repository: tm runs Vale only with a .vale.ini, _vale.ini, \
                 vale.ini, .vale or _vale inside it, or a config the rule names inside it",
                file.display()
            ),
        });
    }
    let style = check.split_once('.').map_or(check, |(style, _)| style);
    if config.styles.iter().any(|named| named == style) {
        return None;
    }
    let root = config.files.first().map_or("the repository's Vale config".to_string(), |file| file.display().to_string());
    Some(Stop::NotAvailable {
        why: format!(
            "no BasedOnStyles in {root} names the style `{style}`: tm never turns on a Vale style the repository did not choose"
        ),
    })
}

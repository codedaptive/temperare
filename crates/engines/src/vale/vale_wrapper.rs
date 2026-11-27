//! `vale_wrapper`: the config cctao hands Vale in place of the repository's own.

use super::ValeConfig;

/// The text of the wrapper config for `style`, from the repository's `config`, one setting a line:
/// - `StylesPath = <the first of config.paths>`, absolute, so the repository's styles and vocabularies are found from
///   the temporary folder the wrapper is written in; left out when Vale resolved none;
/// - `MinAlertLevel = suggestion`, so an alert of every level is reported and the rule file, not the config, decides
///   what counts;
/// - `Vocab = <config.vocab, comma-separated>`, the repository's vocabularies, a stated limit (S-81 (2)); left out when
///   there are none;
/// - a blank line, then `[*]` and `BasedOnStyles = <style>`: the check's style for every file cctao passes.
///
/// Only `StylesPath` and `Vocab` come from the repository, so its skip keys (`BlockIgnores`, `TokenIgnores`,
/// `IgnoredScopes`, `SkippedScopes`, `IgnoredClasses`), its sections and their `Style.Rule = NO` lines never reach
/// Vale (S-67 (1), S-81): `ValeConfig` holds only styles, vocab, paths and files, so nothing else can be written, and
/// vale_wrapper_tests.rs pins the exact text. Err, naming the value, when a value written holds a line break (it would
/// start another setting) or is a path that is not UTF-8.
pub(crate) fn vale_wrapper(config: &ValeConfig, style: &str) -> Result<String, String> {
    let mut text = String::new();
    if let Some(path) = config.paths.first() {
        let shown = path.display();
        let path = path.to_str().ok_or_else(|| format!("the Vale StylesPath {shown} is not UTF-8, so cctao cannot write it into a config"))?;
        text.push_str(&format!("StylesPath = {}\n", one_line(path)?));
    }
    text.push_str("MinAlertLevel = suggestion\n");
    if !config.vocab.is_empty() {
        let names = config.vocab.iter().map(|name| one_line(name)).collect::<Result<Vec<_>, _>>()?;
        text.push_str(&format!("Vocab = {}\n", names.join(", ")));
    }
    text.push_str(&format!("\n[*]\nBasedOnStyles = {}\n", one_line(style)?));
    Ok(text)
}

/// `value` when it holds no line break; else Err naming it.
fn one_line(value: &str) -> Result<&str, String> {
    if value.contains(['\n', '\r']) {
        return Err(format!("the Vale config value {value:?} holds a line break, so cctao cannot write it into a config"));
    }
    Ok(value)
}

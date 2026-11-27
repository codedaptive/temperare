//! `check_off_why`: why a markdownlint config turns a rule off, if it does.

use super::MdRule;

/// The why when the config named `config` (its path relative to the rule file's folder), whose top-level keys and
/// their on/off values are `settings` in file order (`json_settings`, `yaml_settings`), turns `rule` off; None when it
/// leaves it on. As markdownlint 0.42.2's `getEffectiveConfig` decides: the first key equal to `default`, ignoring
/// case, sets every rule's baseline (on when there is none); then each key equal to one of the rule's names or tags,
/// ignoring case, sets it, the last such key winning. Other keys are not markdownlint's and change nothing. The why
/// names the config, the rule and the key that decided it as the config writes it.
pub(crate) fn check_off_why(rule: &MdRule, config: &str, settings: &[(String, bool)]) -> Option<String> {
    let sets = |key: &str| rule.names.iter().chain(rule.tags).any(|name| name.eq_ignore_ascii_case(key));
    let baseline = settings.iter().find(|(key, _)| key.eq_ignore_ascii_case("default"));
    let (key, on) = settings.iter().rev().find(|(key, _)| sets(key)).or(baseline)?;
    (!on).then(|| {
        format!(
            "{config} turns {} off with its `{key}` setting: cctao runs markdownlint with the repository's own config, so \
             the rule would report nothing",
            rule.names[0]
        )
    })
}

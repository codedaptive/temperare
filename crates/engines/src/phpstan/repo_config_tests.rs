//! ZUT for repo_config.rs: the rule's config wins unchecked; else the six names PHPStan 2.3.7 discovers, in its order
//! (each adjacent pair of that order is a row), through a fake `exists` that reads no disk.

use super::repo_config;
use std::path::{Path, PathBuf};

#[test]
fn repo_config_cases() {
    let rows: [(&str, Option<&str>, &[&str], Option<&str>); 11] = [
        ("rule config, absent", Some("conf/strict.neon"), &[], Some("/w/php/conf/strict.neon")),
        ("rule config over a present phpstan.neon", Some("conf/strict.neon"), &["/w/php/phpstan.neon"], Some("/w/php/conf/strict.neon")),
        ("dot-prefixed .phpstan.neon alone", None, &["/w/php/.phpstan.neon"], Some("/w/php/.phpstan.neon")),
        (".phpstan.neon before phpstan.neon", None, &["/w/php/phpstan.neon", "/w/php/.phpstan.neon"], Some("/w/php/.phpstan.neon")),
        ("phpstan.neon before .phpstan.neon.dist", None, &["/w/php/.phpstan.neon.dist", "/w/php/phpstan.neon"], Some("/w/php/phpstan.neon")),
        (".phpstan.neon.dist before phpstan.neon.dist", None, &["/w/php/phpstan.neon.dist", "/w/php/.phpstan.neon.dist"], Some("/w/php/.phpstan.neon.dist")),
        ("phpstan.neon.dist before .phpstan.dist.neon", None, &["/w/php/.phpstan.dist.neon", "/w/php/phpstan.neon.dist"], Some("/w/php/phpstan.neon.dist")),
        (".phpstan.dist.neon before phpstan.dist.neon", None, &["/w/php/phpstan.dist.neon", "/w/php/.phpstan.dist.neon"], Some("/w/php/.phpstan.dist.neon")),
        ("phpstan.dist.neon alone", None, &["/w/php/phpstan.dist.neon"], Some("/w/php/phpstan.dist.neon")),
        ("another name is not looked for", None, &["/w/php/phpstan.neon.local"], None),
        ("none", None, &[], None),
    ];
    for (label, rule_config, present, expected) in rows {
        let exists = |path: &Path| present.iter().any(|p| Path::new(p) == path);
        assert_eq!(repo_config(rule_config, Path::new("/w/php"), &exists), expected.map(PathBuf::from), "{label}");
    }
}

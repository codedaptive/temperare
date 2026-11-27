//! Unit test for actionlint_config.rs: the rule's config joined to the folder whether or not it exists; else the top's
//! `.github/actionlint.yaml`, then `.yml`; else None. Nothing on disk is read: `exists` is a list.

use super::actionlint_config;
use std::path::{Path, PathBuf};

#[test]
fn actionlint_config_cases() {
    let rows: [(&str, Option<&str>, &[&str], Option<&str>); 6] = [
        ("rule's config, absent", Some("lint/al.yaml"), &[], Some("/r/sub/lint/al.yaml")),
        ("rule's config over the top's", Some("al.yaml"), &["/r/.github/actionlint.yaml"], Some("/r/sub/al.yaml")),
        ("yaml", None, &["/r/.github/actionlint.yaml"], Some("/r/.github/actionlint.yaml")),
        ("yml", None, &["/r/.github/actionlint.yml"], Some("/r/.github/actionlint.yml")),
        ("both: yaml wins", None, &["/r/.github/actionlint.yml", "/r/.github/actionlint.yaml"], Some("/r/.github/actionlint.yaml")),
        ("the folder's own is not read", None, &["/r/sub/.github/actionlint.yaml"], None),
    ];
    for (label, rule_config, present, expected) in rows {
        let exists = |path: &Path| present.iter().any(|p| Path::new(p) == path);
        let got = actionlint_config(rule_config, Path::new("/r/sub"), Path::new("/r"), &exists);
        assert_eq!(got, expected.map(PathBuf::from), "{label}");
    }
}

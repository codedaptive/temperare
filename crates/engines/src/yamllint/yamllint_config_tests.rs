//! Unit test for yamllint_config.rs: the rule's config wins unchecked; else yamllint 1.38.0's three names in its order, the
//! nearest folder first, never above the repository top, through a fake `exists` that reads no disk.

use super::yamllint_config;
use std::path::{Path, PathBuf};

#[test]
fn yamllint_config_cases() {
    let rows: [(&str, Option<&str>, &[&str], Option<&str>); 11] = [
        ("rule config, absent", Some("conf/lint.yaml"), &[], Some("/w/top/sub/conf/lint.yaml")),
        ("rule config over a present .yamllint", Some("conf/lint.yaml"), &["/w/top/sub/.yamllint"], Some("/w/top/sub/conf/lint.yaml")),
        (".yamllint alone", None, &["/w/top/sub/.yamllint"], Some("/w/top/sub/.yamllint")),
        (".yamllint before .yamllint.yaml", None, &["/w/top/sub/.yamllint.yaml", "/w/top/sub/.yamllint"], Some("/w/top/sub/.yamllint")),
        (".yamllint.yaml before .yamllint.yml", None, &["/w/top/sub/.yamllint.yml", "/w/top/sub/.yamllint.yaml"], Some("/w/top/sub/.yamllint.yaml")),
        (".yamllint.yml alone", None, &["/w/top/sub/.yamllint.yml"], Some("/w/top/sub/.yamllint.yml")),
        ("the top's config", None, &["/w/top/.yamllint"], Some("/w/top/.yamllint")),
        ("nearest folder's last name before the top's first", None, &["/w/top/.yamllint", "/w/top/sub/.yamllint.yml"], Some("/w/top/sub/.yamllint.yml")),
        ("above the top is not looked at", None, &["/w/.yamllint"], None),
        ("another name is not looked for", None, &["/w/top/sub/yamllint.yaml"], None),
        ("none", None, &[], None),
    ];
    for (label, rule_config, present, expected) in rows {
        let exists = |path: &Path| present.iter().any(|p| Path::new(p) == path);
        let got = yamllint_config(rule_config, Path::new("/w/top/sub"), Path::new("/w/top"), &exists);
        assert_eq!(got, expected.map(PathBuf::from), "{label}");
    }
}

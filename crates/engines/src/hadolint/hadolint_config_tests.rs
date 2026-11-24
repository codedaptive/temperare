//! ZUT for hadolint_config.rs: the rule's config joined to the folder whatever exists; else `.hadolint.yaml` before
//! `.hadolint.yml`, in the folder only; else None.

use super::hadolint_config;
use std::path::{Path, PathBuf};

#[test]
fn hadolint_config_cases() {
    let rows: [(&str, Option<&str>, Vec<&str>, Option<&str>); 7] = [
        ("the rule's config", Some("ci/hadolint.yaml"), vec!["/r/.hadolint.yaml"], Some("/r/ci/hadolint.yaml")),
        ("the rule's config, absent", Some("missing.yaml"), vec![], Some("/r/missing.yaml")),
        (".hadolint.yaml", None, vec!["/r/.hadolint.yaml"], Some("/r/.hadolint.yaml")),
        (".hadolint.yml", None, vec!["/r/.hadolint.yml"], Some("/r/.hadolint.yml")),
        (".yaml before .yml", None, vec!["/r/.hadolint.yml", "/r/.hadolint.yaml"], Some("/r/.hadolint.yaml")),
        ("a parent's config", None, vec!["/.hadolint.yaml"], None),
        ("none", None, vec![], None),
    ];
    for (label, config, present, expected) in rows {
        let exists = |path: &Path| present.iter().any(|at| Path::new(at) == path);
        assert_eq!(hadolint_config(Path::new("/r"), config, &exists), expected.map(PathBuf::from), "{label}");
    }
}

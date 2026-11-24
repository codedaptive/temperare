//! ZUT for is_yaml_source.rs: `.yaml`, `.yml` and a file named `.yamllint` are sources; any other name is not.

use super::is_yaml_source;
use std::path::Path;

#[test]
fn is_yaml_source_cases() {
    let rows = [
        ("yaml", "a.yaml", true),
        ("yml in a folder", "d/b.yml", true),
        ("yamllint config", ".yamllint", true),
        ("yamllint config in a folder", "d/.yamllint", true),
        ("yamllint config with yaml extension", ".yamllint.yaml", true),
        ("named only .yaml", ".yaml", true),
        ("named only .yml in a folder", "d/.yml", true),
        ("upper case", "a.YAML", false),
        ("json", "a.json", false),
        ("no extension", "yaml", false),
        ("yamllint as an extension", "a.yamllint", false),
        ("yamllint config name in a longer name", "x.yamllint.bak", false),
    ];
    for (label, path, expected) in rows {
        assert_eq!(is_yaml_source(Path::new(path)), expected, "{label}");
    }
}

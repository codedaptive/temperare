use super::remote_dependencies;
use procio::Output;

#[test]
fn remote_dependencies_cases() {
    let rows: [(&str, &[u8], Option<Vec<&str>>); 4] = [
        ("remote and local", include_bytes!("fixtures/dump_package_remote.json"), Some(vec!["dep"])),
        ("remote and local binary targets", include_bytes!("fixtures/dump_package_binary.json"), Some(vec!["binary target Bin (https://example.invalid/Bin.xcframework.zip)"])),
        ("no dependencies", include_bytes!("fixtures/dump_package_clean.json"), Some(vec![])),
        ("not JSON", b"not JSON", None),
    ];
    for (label, stdout, expected) in rows {
        let out = Output { code: Some(0), stdout: stdout.to_vec(), stderr: vec![] };
        match expected {
            Some(expected) => assert_eq!(remote_dependencies(&out).unwrap(), expected, "{label}"),
            None => assert!(remote_dependencies(&out).is_err(), "{label}"),
        }
    }
}

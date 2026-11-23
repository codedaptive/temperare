//! ZUT for missing_packages.rs on dart 3.14.1's real output (fixtures/missing_packages.txt and fixtures/machine.txt,
//! made as fixtures/commands.txt says): an unfetched package URI with no package config is NotAvailable naming it and
//! `dart pub get`; a package config, a relative URI or no such line is None.

use super::missing_packages;
use crate::types::Stop;
use procio::Output;

#[test]
fn missing_packages_cases() {
    let missing = include_str!("fixtures/missing_packages.txt");
    let include = missing.lines().nth(1).unwrap();
    let machine = include_str!("fixtures/machine.txt");
    let rows: [(&str, &str, bool, Option<&str>); 6] = [
        ("import and include", missing, false, Some("`package:path/path.dart`")),
        ("include only", include, false, Some("`package:lints/recommended.yaml`")),
        ("package config present", missing, true, None),
        ("relative uri, no package config", machine, false, None),
        ("relative uri, package config present", machine, true, None),
        ("nothing printed", "", false, None),
    ];
    let mut failed = Vec::new();
    for (name, stdout, has_package_config, uri) in rows {
        let result = std::panic::catch_unwind(|| {
            let out = Output { code: Some(3), stdout: stdout.as_bytes().to_vec(), stderr: vec![] };
            match (missing_packages(&out, has_package_config), uri) {
                (None, None) => {}
                (Some(Stop::NotAvailable { why }), Some(uri)) => {
                    assert!(why.contains(uri), "{name}: {why:?}");
                    assert!(why.contains("`dart pub get`"), "{name}: {why:?}");
                }
                (got, _) => panic!("{name}: {got:?}"),
            }
        });
        if result.is_err() { failed.push(name); }
    }
    assert!(failed.is_empty(), "failed rows: {failed:?}");
}

//! ZUT for package_path.rs: a `package:` URI resolved through a package_config.json written out by hand in the shape
//! `dart pub get` writes (configVersion 2), and the cases that do not resolve.

use super::package_path;
use std::path::{Path, PathBuf};

const CONFIG: &str = r#"{
  "configVersion": 2,
  "packages": [
    {"name": "lints", "rootUri": "file:///home/u/.pub-cache/hosted/pub.dev/lints-6.0.0", "packageUri": "lib/", "languageVersion": "3.6"},
    {"name": "spaced", "rootUri": "file:///home/u/my%20pkgs/spaced", "languageVersion": "3.0"},
    {"name": "probe", "rootUri": "../", "packageUri": "lib/", "languageVersion": "3.0"},
    {"name": "remote", "rootUri": "file://host/share/remote"},
    {"name": "web", "rootUri": "https://example.com/web"},
    {"name": "numbered", "rootUri": 7}
  ]
}"#;

#[test]
fn package_path_cases() {
    let dir = Path::new("/w/pkg/.dart_tool");
    let rows: [(&str, &str, &str, Option<&str>); 11] = [
        ("file uri", "package:lints/recommended.yaml", CONFIG,
            Some("/home/u/.pub-cache/hosted/pub.dev/lints-6.0.0/lib/recommended.yaml")),
        ("default packageUri, escape", "package:spaced/a%20b.yaml", CONFIG, Some("/home/u/my pkgs/spaced/lib/a b.yaml")),
        ("escape with a sign", "package:lints/a%+f.yaml", CONFIG, None),
        ("relative rootUri", "package:probe/options.yaml", CONFIG, Some("/w/pkg/.dart_tool/../lib/options.yaml")),
        ("unknown package", "package:flutter_lints/flutter.yaml", CONFIG, None),
        ("not a package uri", "lints/recommended.yaml", CONFIG, None),
        ("no path", "package:lints", CONFIG, None),
        ("file uri with a host", "package:remote/a.yaml", CONFIG, None),
        ("other scheme", "package:web/a.yaml", CONFIG, None),
        ("rootUri not a string", "package:numbered/a.yaml", CONFIG, None),
        ("config not json", "package:lints/recommended.yaml", "packages:", None),
    ];
    let mut failed = Vec::new();
    for (name, uri, config, want) in rows {
        let got = package_path(uri, config, dir);
        if got != want.map(PathBuf::from) {
            eprintln!("{name}: got {got:?}");
            failed.push(name);
        }
    }
    assert!(failed.is_empty(), "failed rows: {failed:?}");
}

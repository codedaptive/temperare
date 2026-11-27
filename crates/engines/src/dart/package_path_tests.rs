//! ZUT for package_path.rs: a `package:` URI resolved through a package_config.json written out by hand in the shape
//! `dart pub get` writes (configVersion 2), and the cases that do not resolve. A row's `windows` column gives the
//! platform as data, so the Windows rows hold on every host.

use super::package_path;
use std::path::{Path, PathBuf};

const CONFIG: &str = r#"{
  "configVersion": 2,
  "packages": [
    {"name": "lints", "rootUri": "file:///home/u/.pub-cache/hosted/pub.dev/lints-6.0.0", "packageUri": "lib/", "languageVersion": "3.6"},
    {"name": "spaced", "rootUri": "file:///home/u/my%20pkgs/spaced", "languageVersion": "3.0"},
    {"name": "probe", "rootUri": "../", "packageUri": "lib/", "languageVersion": "3.0"},
    {"name": "drive", "rootUri": "file:///C:/x"},
    {"name": "lowdrive", "rootUri": "file:///c:/a%20b"},
    {"name": "root", "rootUri": "file:///C:/"},
    {"name": "encdrive", "rootUri": "file:///C%3A/x"},
    {"name": "remote", "rootUri": "file://host/share/remote"},
    {"name": "unc", "rootUri": "file:////server/share/x"},
    {"name": "web", "rootUri": "https://example.com/web"},
    {"name": "numbered", "rootUri": 7}
  ]
}"#;

#[test]
fn package_path_cases() {
    // A Windows row's dir is the canonical (verbatim) folder Windows gives config_dir; only the relative row reads it.
    let dir = |windows| Path::new(if windows { r"\\?\C:\w\pkg\.dart_tool" } else { "/w/pkg/.dart_tool" });
    let rows: [(&str, &str, &str, bool, Option<&str>); 19] = [
        ("drive file uri", "package:drive/a.yaml", CONFIG, true, Some(r"C:\x\lib\a.yaml")),
        ("lowercase drive with escape", "package:lowdrive/b.yaml", CONFIG, true, Some(r"c:\a b\lib\b.yaml")),
        ("drive root", "package:root/a.yaml", CONFIG, true, Some(r"C:\lib\a.yaml")),
        ("escaped drive colon", "package:encdrive/a.yaml", CONFIG, true, Some(r"C:\x\lib\a.yaml")),
        ("relative rootUri, windows", "package:probe/options.yaml", CONFIG, true,
            Some(r"\\?\C:\w\pkg\lib\options.yaml")),
        ("drive file uri, unix", "package:drive/a.yaml", CONFIG, false, Some("/C:/x/lib/a.yaml")),
        ("file uri", "package:lints/recommended.yaml", CONFIG, false,
            Some("/home/u/.pub-cache/hosted/pub.dev/lints-6.0.0/lib/recommended.yaml")),
        ("default packageUri, escape", "package:spaced/a%20b.yaml", CONFIG, false,
            Some("/home/u/my pkgs/spaced/lib/a b.yaml")),
        ("escape with a sign", "package:lints/a%+f.yaml", CONFIG, false, None),
        ("relative rootUri", "package:probe/options.yaml", CONFIG, false, Some("/w/pkg/.dart_tool/../lib/options.yaml")),
        ("unknown package", "package:flutter_lints/flutter.yaml", CONFIG, false, None),
        ("not a package uri", "lints/recommended.yaml", CONFIG, false, None),
        ("no path", "package:lints", CONFIG, false, None),
        ("file uri with a host", "package:remote/a.yaml", CONFIG, false, None),
        ("four-slash UNC rootUri, windows", "package:unc/a.yaml", CONFIG, true, None),
        ("four-slash UNC rootUri, unix", "package:unc/a.yaml", CONFIG, false, None),
        ("other scheme", "package:web/a.yaml", CONFIG, false, None),
        ("rootUri not a string", "package:numbered/a.yaml", CONFIG, false, None),
        ("config not json", "package:lints/recommended.yaml", "packages:", false, None),
    ];
    let mut failed = Vec::new();
    for (name, uri, config, windows, want) in rows {
        let got = package_path(uri, config, dir(windows), windows);
        if got != want.map(PathBuf::from) {
            eprintln!("{name}: got {got:?}");
            failed.push(name);
        }
    }
    assert!(failed.is_empty(), "failed rows: {failed:?}");
}

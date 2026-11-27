//! ZUT for probe_files.rs: the probe package's files for one check, as fixtures/commands.txt records them.

use super::probe_files;

#[test]
fn probe_files_cases() {
    let rows: [(&str, &str); 2] = [
        ("prefer_single_quotes",
            "analyzer:\n  errors:\n    prefer_single_quotes: warning\nlinter:\n  rules:\n    - prefer_single_quotes\n"),
        ("unused_local_variable",
            "analyzer:\n  errors:\n    unused_local_variable: warning\nlinter:\n  rules:\n    - unused_local_variable\n"),
    ];
    for (check, options) in rows {
        let [(pubspec, pubspec_text), (name, text)] = probe_files(check);
        assert_eq!((pubspec, pubspec_text.as_str()), ("pubspec.yaml", "name: cctao_probe\nenvironment:\n  sdk: ^3.0.0\n"), "{check}");
        assert_eq!((name, text.as_str()), ("analysis_options.yaml", options), "{check}");
    }
}

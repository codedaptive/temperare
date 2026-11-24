//! `probe_files`: the package the probe run analyzes to learn what kind of code a check is.

/// The probe package's two files, each a name and its text: `pubspec.yaml` (package `tm_probe`, any Dart 3 SDK, no
/// dependency, so nothing is fetched) and `analysis_options.yaml`, which sets `check` as an error code
/// (`analyzer: errors: <check>: warning`) and enables it as a lint (`linter: rules: - <check>`). Dart warns on each
/// it does not know as that (fixtures/commands.txt), which `kind_of` reads. Pure.
pub(crate) fn probe_files(check: &str) -> [(&'static str, String); 2] {
    [
        ("pubspec.yaml", "name: tm_probe\nenvironment:\n  sdk: ^3.0.0\n".to_string()),
        (
            "analysis_options.yaml",
            format!("analyzer:\n  errors:\n    {check}: warning\nlinter:\n  rules:\n    - {check}\n"),
        ),
    ]
}

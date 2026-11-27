//! Unit test for ini_settings.rs: each kept key, global and per-module sections, the truth words, comments and continuation
//! lines, the first separator, `[DEFAULT]` kept as per-module, a dashed key and other sections dropped, file order.

use super::{ini_settings, MypySetting};

fn set(section: &str, per_module: bool, key: &str, flag: bool, codes: &[&str]) -> MypySetting {
    let codes = codes.iter().map(|code| code.to_string()).collect();
    MypySetting { section: section.into(), per_module, key: key.into(), flag, codes }
}

#[test]
fn ini_settings_reads_mypy_sections_as_configparser_does() {
    let rows: Vec<(&str, &str, Vec<MypySetting>)> = vec![
        ("global flag", "[mypy]\nignore_errors = True\n", vec![set("[mypy]", false, "ignore_errors", true, &[])]),
        (
            "per-module flag with a colon",
            "[mypy-pkg.*]\nignore_missing_imports: yes\n",
            vec![set("[mypy-pkg.*]", true, "ignore_missing_imports", true, &[])],
        ),
        (
            "truth words",
            "[mypy]\nignore_errors = on\nignore_errors = maybe\n",
            vec![set("[mypy]", false, "ignore_errors", true, &[]), set("[mypy]", false, "ignore_errors", false, &[])],
        ),
        (
            "code list",
            "[mypy-a]\ndisable_error_code = arg-type, ,name-defined\n",
            vec![set("[mypy-a]", true, "disable_error_code", false, &["arg-type", "name-defined"])],
        ),
        (
            "continuation lines past a comment and a blank line",
            "[mypy-a]\ndisable_error_code =\n    arg-type,\n    # note\n\n    misc\nignore_errors = False\n",
            vec![
                set("[mypy-a]", true, "disable_error_code", false, &["arg-type", "misc"]),
                set("[mypy-a]", true, "ignore_errors", false, &[]),
            ],
        ),
        ("key lowercased", "[mypy]\nIGNORE_Errors = true\n", vec![set("[mypy]", false, "ignore_errors", true, &[])]),
        ("split at the first separator", "[mypy]\ndisable_error_code = a:b\n", vec![set("[mypy]", false, "disable_error_code", false, &["a:b"])]),
        ("no inline comment", "[mypy]\nignore_errors = True # no\n", vec![set("[mypy]", false, "ignore_errors", false, &[])]),
        ("DEFAULT flag", "[DEFAULT]\nignore_errors = yes\n[mypy]\n", vec![set("[DEFAULT]", true, "ignore_errors", true, &[])]),
        (
            "DEFAULT code list",
            "[DEFAULT]\ndisable_error_code = arg-type\n",
            vec![set("[DEFAULT]", true, "disable_error_code", false, &["arg-type"])],
        ),
        ("dashed key dropped", "[mypy]\nignore-errors = True\n", vec![]),
        (
            "other sections, comments and other keys skipped",
            "ignore_errors = True\n[tool]\nignore_errors = True\n[MYPY]\nignore_errors = True\n[mypy]\n; ignore_errors = True\n  # ignore_errors = True\nstrict = True\n",
            vec![],
        ),
        (
            "file order",
            "[mypy-b]\nignore_errors = True\n[mypy]\nignore_missing_imports = True\n",
            vec![set("[mypy-b]", true, "ignore_errors", true, &[]), set("[mypy]", false, "ignore_missing_imports", true, &[])],
        ),
        ("empty text", "", vec![]),
    ];
    let failed: Vec<String> = rows
        .into_iter()
        .filter_map(|(label, text, want)| {
            let got = ini_settings(text);
            (got != want).then(|| format!("{label}: got {got:?}"))
        })
        .collect();
    assert!(failed.is_empty(), "failed rows: {failed:#?}");
}

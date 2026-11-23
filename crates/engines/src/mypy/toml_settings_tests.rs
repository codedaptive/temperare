//! ZUT for toml_settings.rs: the global table and numbered overrides, bool and string flags, code arrays and code
//! strings, file order, exact keys, no `[tool.mypy]`, and text that is not TOML.

use super::{toml_settings, MypySetting};

fn set(section: &str, per_module: bool, key: &str, flag: bool, codes: &[&str]) -> MypySetting {
    let codes = codes.iter().map(|code| code.to_string()).collect();
    MypySetting { section: section.into(), per_module, key: key.into(), flag, codes }
}

#[test]
fn toml_settings_reads_tool_mypy_and_its_overrides() {
    let rows: Vec<(&str, &str, Result<Vec<MypySetting>, &str>)> = vec![
        ("global bool", "[tool.mypy]\nignore_errors = true\n", Ok(vec![set("[tool.mypy]", false, "ignore_errors", true, &[])])),
        (
            "overrides numbered from 1",
            "[tool.mypy]\nstrict = true\n[[tool.mypy.overrides]]\nmodule = \"a\"\nignore_missing_imports = true\n\
             [[tool.mypy.overrides]]\nmodule = \"b\"\ndisable_error_code = [\"arg-type\", \"misc\"]\n",
            Ok(vec![
                set("[[tool.mypy.overrides]] #1", true, "ignore_missing_imports", true, &[]),
                set("[[tool.mypy.overrides]] #2", true, "disable_error_code", false, &["arg-type", "misc"]),
            ]),
        ),
        (
            "string flags",
            "[tool.mypy]\nignore_errors = \"Yes\"\nignore_missing_imports = \"off\"\n",
            Ok(vec![
                set("[tool.mypy]", false, "ignore_errors", true, &[]),
                set("[tool.mypy]", false, "ignore_missing_imports", false, &[]),
            ]),
        ),
        (
            "string code list",
            "[tool.mypy]\ndisable_error_code = \"arg-type, misc,\"\n",
            Ok(vec![set("[tool.mypy]", false, "disable_error_code", false, &["arg-type", "misc"])]),
        ),
        (
            "file order within a table",
            "[tool.mypy]\nignore_missing_imports = false\ndisable_error_code = [\"misc\"]\n",
            Ok(vec![
                set("[tool.mypy]", false, "ignore_missing_imports", false, &[]),
                set("[tool.mypy]", false, "disable_error_code", false, &["misc"]),
            ]),
        ),
        ("dashed and cased keys dropped", "[tool.mypy]\nignore-errors = true\nIgnore_Errors = true\n", Ok(vec![])),
        ("no tool.mypy", "[tool.ruff]\nline-length = 88\n", Ok(vec![])),
        ("not TOML", "[tool.mypy\n", Err("TOML parse error")),
    ];
    let failed: Vec<String> = rows
        .into_iter()
        .filter_map(|(label, text, want)| {
            let got = toml_settings(text);
            let matches = match (&got, &want) {
                (Ok(settings), Ok(want)) => settings == want,
                (Err(error), Err(start)) => error.starts_with(start),
                _ => false,
            };
            (!matches).then(|| format!("{label}: got {got:?}"))
        })
        .collect();
    assert!(failed.is_empty(), "failed rows: {failed:#?}");
}

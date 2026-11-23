//! ZUT for config_suppression.rs: each silencing setting with its why, a global disable_error_code that does not
//! silence, flags that are false or do not touch the check, the first in file order, and TOML that does not parse.

use super::config_suppression;

const TAIL: &str = "silences mypy findings outside the rule file; exceptions go in the rule file";

#[test]
fn config_suppression_names_the_first_silencing_setting() {
    let rows: [(&str, &str, &str, &str, Result<Option<&str>, &str>); 13] = [
        ("per-module ignore_errors", "mypy.ini", "[mypy]\n[mypy-a]\nignore_errors = True\n", "arg-type", Ok(Some("mypy config mypy.ini: [mypy-a] ignore_errors"))),
        ("DEFAULT ignore_errors", "mypy.ini", "[DEFAULT]\nignore_errors = True\n[mypy]\n", "arg-type", Ok(Some("mypy config mypy.ini: [DEFAULT] ignore_errors"))),
        ("global ignore_errors in toml", "pyproject.toml", "[tool.mypy]\nignore_errors = true\n", "arg-type", Ok(Some("mypy config pyproject.toml: [tool.mypy] ignore_errors"))),
        ("ignore_errors false", "mypy.ini", "[mypy]\nignore_errors = False\n", "arg-type", Ok(None)),
        ("global disable_error_code", "setup.cfg", "[mypy]\ndisable_error_code = arg-type\n", "arg-type", Ok(None)),
        ("per-module disable_error_code", "sub/mypy.ini", "[mypy-pkg.*]\ndisable_error_code = misc, arg-type\n", "arg-type", Ok(Some("mypy config sub/mypy.ini: [mypy-pkg.*] disable_error_code"))),
        ("per-module disable of another code", "sub/mypy.ini", "[mypy-pkg.*]\ndisable_error_code = misc\n", "arg-type", Ok(None)),
        (
            "override disable_error_code in toml",
            "pyproject.toml",
            "[[tool.mypy.overrides]]\nmodule = \"a\"\ndisable_error_code = [\"arg-type\"]\n",
            "arg-type",
            Ok(Some("mypy config pyproject.toml: [[tool.mypy.overrides]] #1 disable_error_code")),
        ),
        ("ignore_missing_imports, import-not-found", ".mypy.ini", "[mypy]\nignore_missing_imports = True\n", "import-not-found", Ok(Some("mypy config .mypy.ini: [mypy] ignore_missing_imports"))),
        ("ignore_missing_imports, import-untyped", ".mypy.ini", "[mypy]\nignore_missing_imports = True\n", "import-untyped", Ok(Some("mypy config .mypy.ini: [mypy] ignore_missing_imports"))),
        ("ignore_missing_imports, another check", ".mypy.ini", "[mypy]\nignore_missing_imports = True\n", "arg-type", Ok(None)),
        (
            "first in file order",
            "mypy.ini",
            "[mypy-a]\nignore_missing_imports = True\n[mypy-b]\nignore_errors = True\n",
            "import-untyped",
            Ok(Some("mypy config mypy.ini: [mypy-a] ignore_missing_imports")),
        ),
        ("TOML that does not parse", "pyproject.toml", "[tool.mypy\n", "arg-type", Err("mypy config pyproject.toml cannot be read: TOML parse error")),
    ];
    let failed: Vec<String> = rows
        .into_iter()
        .filter_map(|(label, name, text, check, want)| {
            let got = config_suppression(name, text, check);
            let matches = match (&got, want) {
                (Ok(why), Ok(head)) => *why == head.map(|head| format!("{head} {TAIL}")),
                (Err(error), Err(start)) => error.starts_with(start),
                _ => false,
            };
            (!matches).then(|| format!("{label}: got {got:?}"))
        })
        .collect();
    assert!(failed.is_empty(), "failed rows: {failed:#?}");
}

//! Unit test for mypy_config.rs: mypy's order within a directory, a pyproject.toml without `tool.mypy` and a setup.cfg
//! without `[mypy]` skipped, the walk up to and including the top and no further, a folder outside the top alone.

use super::mypy_config;
use std::path::{Path, PathBuf};

const TOOL_MYPY: &str = "[tool.mypy]\nstrict = true\n";

#[test]
fn mypy_config_finds_the_file_mypy_reads() {
    let rows: [(&str, &str, &str, Vec<(&str, &str)>, Option<&str>); 12] = [
        (
            "mypy.ini first",
            "/r",
            "/r",
            vec![("/r/mypy.ini", "x"), ("/r/.mypy.ini", ""), ("/r/pyproject.toml", TOOL_MYPY), ("/r/setup.cfg", "[mypy]\n")],
            Some("/r/mypy.ini"),
        ),
        ("an empty .mypy.ini before pyproject.toml", "/r", "/r", vec![("/r/.mypy.ini", ""), ("/r/pyproject.toml", TOOL_MYPY)], Some("/r/.mypy.ini")),
        ("pyproject.toml before setup.cfg", "/r", "/r", vec![("/r/pyproject.toml", TOOL_MYPY), ("/r/setup.cfg", "[mypy]\n")], Some("/r/pyproject.toml")),
        ("pyproject.toml without tool.mypy", "/r", "/r", vec![("/r/pyproject.toml", "[tool.ruff]\n"), ("/r/setup.cfg", "[mypy]\n")], Some("/r/setup.cfg")),
        ("pyproject.toml that is not TOML", "/r", "/r", vec![("/r/pyproject.toml", "[tool.mypy\n")], None),
        ("setup.cfg without [mypy]", "/r", "/r", vec![("/r/setup.cfg", "[flake8]\n[mypy-a]\n")], None),
        ("walks up to a trimmed [mypy] line", "/r/a/b", "/r", vec![("/r/a/setup.cfg", "  [mypy]  \n"), ("/r/mypy.ini", "")], Some("/r/a/setup.cfg")),
        ("the nearer directory wins", "/r/a", "/r", vec![("/r/a/pyproject.toml", TOOL_MYPY), ("/r/mypy.ini", "")], Some("/r/a/pyproject.toml")),
        ("the top is searched", "/r/a", "/r", vec![("/r/mypy.ini", "")], Some("/r/mypy.ini")),
        ("stops at the top", "/r/a/b", "/r/a", vec![("/r/mypy.ini", "")], None),
        ("outside the top: the folder alone", "/x/a", "/r", vec![("/x/a/.mypy.ini", ""), ("/x/mypy.ini", "")], Some("/x/a/.mypy.ini")),
        ("outside the top: no walk up", "/x/a", "/r", vec![("/x/mypy.ini", ""), ("/r/mypy.ini", "")], None),
    ];
    let failed: Vec<String> = rows
        .into_iter()
        .filter_map(|(label, folder, top, files, want)| {
            let read = |path: &Path| files.iter().find(|(at, _)| Path::new(at) == path).map(|(_, text)| text.to_string());
            let got = mypy_config(Path::new(folder), Path::new(top), &read);
            (got != want.map(PathBuf::from)).then(|| format!("{label}: got {got:?}"))
        })
        .collect();
    assert!(failed.is_empty(), "failed rows: {failed:#?}");
}

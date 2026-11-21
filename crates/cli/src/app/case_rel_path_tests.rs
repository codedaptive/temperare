//! Unit tests for case_rel_path.rs: names that could leave the case folder (absolute, a drive, a `..` part, empty) are
//! refused, and kept names come back as their non-empty, non-`.` parts joined by `/`.

use super::case_rel_path::case_rel_path;

fn refused(name: &str) -> Result<String, String> {
    Err(format!("case file \"{name}\" is not a relative path inside the case folder"))
}

#[test]
fn case_rel_path_rows() {
    let rows: Vec<(&str, Result<String, String>)> = vec![
        ("/etc/x", refused("/etc/x")),
        ("\\x", refused("\\x")),
        ("C:x", refused("C:x")),
        ("c:\\x", refused("c:\\x")),
        ("..", refused("..")),
        ("../escape.py", refused("../escape.py")),
        ("a/../../b", refused("a/../../b")),
        ("a\\..\\b", refused("a\\..\\b")),
        ("", refused("")),
        ("a.py", Ok("a.py".to_string())),
        ("src/a.rs", Ok("src/a.rs".to_string())),
        ("src\\a.rs", Ok("src/a.rs".to_string())),
        ("./a.py", Ok("a.py".to_string())),
        ("src//./a.rs", Ok("src/a.rs".to_string())),
        ("src//./a.py", Ok("src/a.py".to_string())),
        ("a..b.py", Ok("a..b.py".to_string())),
        ("ab:c.rs", Ok("ab:c.rs".to_string())),
    ];
    let wrong: Vec<String> = rows
        .into_iter()
        .filter_map(|(name, want)| {
            let got = case_rel_path(name);
            (got != want).then(|| format!("{name:?}: got {got:?}, want {want:?}"))
        })
        .collect();
    assert!(wrong.is_empty(), "rows that fail:\n{}", wrong.join("\n"));
}

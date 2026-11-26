//! ZUT for case_path.rs: a kept name is joined to the folder part by part, a name of only `.` parts is the folder
//! itself, and a refused name gives case_rel_path's why. Paths compare by their exact text, so a trailing separator
//! fails a row; each kept row's path is `/case` joined with its parts, so the separator is the platform's (`/case/a.py`
//! on unix). Pure: the folder named here need not exist.

use super::case_path::case_path;
use std::path::{Path, PathBuf};

#[test]
fn case_path_rows() {
    let case = Path::new("/case");
    let rows: Vec<(&str, Result<PathBuf, String>)> = vec![
        ("a.py", Ok(case.join("a.py"))),
        ("src\\a.rs", Ok(case.join("src").join("a.rs"))),
        ("src//./a.rs", Ok(case.join("src").join("a.rs"))),
        (".", Ok(PathBuf::from("/case"))),
        ("./", Ok(PathBuf::from("/case"))),
        ("../escape.py", Err("case file \"../escape.py\" is not a relative path inside the case folder".to_string())),
    ];
    let wrong: Vec<String> = rows
        .into_iter()
        .filter_map(|(name, want)| {
            let got = case_path(case, name).map(PathBuf::into_os_string);
            let want = want.map(PathBuf::into_os_string);
            (got != want).then(|| format!("{name:?}: got {got:?}, want {want:?}"))
        })
        .collect();
    assert!(wrong.is_empty(), "rows that fail:\n{}", wrong.join("\n"));
}

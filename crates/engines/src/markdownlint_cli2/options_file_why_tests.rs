//! ZUT for options_file_why.rs with a fake `is_file`: an options file in the rule's folder or on the way down to a
//! checked file stops the rule, named relative to the folder; one elsewhere does not. A row's expected path is the one
//! the why must name, None for no why.

use super::options_file_why;
use std::path::{Path, PathBuf};

const FOLDER: &str = "/w/repo";

#[test]
fn options_file_why_cases() {
    let cjs = Path::new("docs").join(".markdownlint-cli2.cjs").display().to_string();
    let api = Path::new("docs/api").join(".markdownlint-cli2.yaml").display().to_string();
    let rows: [(&str, &[&str], &[&str], Option<&str>); 9] = [
        ("none", &["a.md"], &[], None),
        ("in the folder", &["a.md"], &[".markdownlint-cli2.jsonc"], Some(".markdownlint-cli2.jsonc")),
        ("first name in order", &["a.md"], &[".markdownlint-cli2.mjs", ".markdownlint-cli2.yaml"], Some(".markdownlint-cli2.yaml")),
        ("on the way down", &["docs/a.md"], &["docs/.markdownlint-cli2.cjs"], Some(cjs.as_str())),
        ("two levels down", &["a.md", "docs/api/x.md"], &["docs/api/.markdownlint-cli2.yaml"], Some(api.as_str())),
        ("folder before below", &["docs/a.md"], &["docs/.markdownlint-cli2.jsonc", ".markdownlint-cli2.mjs"],
            Some(".markdownlint-cli2.mjs")),
        ("in a sibling folder", &["docs/a.md"], &["other/.markdownlint-cli2.jsonc"], None),
        ("below the checked file", &["docs/a.md"], &["docs/sub/.markdownlint-cli2.jsonc"], None),
        // The parent's file under both spellings a probe could build: the absolute one replaces FOLDER in the join
        // below (`folder.parent()`), and `../` stays a `..` component (`folder.join("..")`).
        ("above the folder", &["a.md"], &["/w/.markdownlint-cli2.jsonc", "../.markdownlint-cli2.jsonc"], None),
    ];
    for (label, files, present, expected) in rows {
        let present: Vec<PathBuf> = present.iter().map(|path| Path::new(FOLDER).join(path)).collect();
        let is_file = |path: &Path| present.iter().any(|known| known == path);
        let files: Vec<PathBuf> = files.iter().map(PathBuf::from).collect();
        let why = options_file_why(Path::new(FOLDER), &files, &is_file);
        match expected {
            None => assert_eq!(why, None, "{label}"),
            Some(path) => assert!(
                why.as_ref().is_some_and(|why| why.starts_with(&format!("markdownlint-cli2 options file {path} "))
                    && why.contains("rule file")),
                "{label}: {why:?}"
            ),
        }
    }
}

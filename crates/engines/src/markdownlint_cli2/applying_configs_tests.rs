//! ZUT for applying_configs.rs with a fake `is_file`: per file the nearest folder holding a config, the first name in
//! markdownlint-cli2's order there, each config once, in the order the files reach them.

use super::applying_configs;
use std::path::{Path, PathBuf};

const FOLDER: &str = "/w/repo";

#[test]
fn applying_configs_cases() {
    let rows: [(&str, &[&str], &[&str], &[&str]); 10] = [
        ("none", &["a.md"], &[], &[]),
        ("one for all", &["a.md", "docs/b.md"], &[".markdownlint.json"], &[".markdownlint.json"]),
        ("json before yaml", &["a.md"], &[".markdownlint.yaml", ".markdownlint.json"], &[".markdownlint.json"]),
        ("jsonc before json", &["a.md"], &[".markdownlint.json", ".markdownlint.jsonc"], &[".markdownlint.jsonc"]),
        ("yml before cjs", &["a.md"], &[".markdownlint.cjs", ".markdownlint.yml"], &[".markdownlint.yml"]),
        ("nearest wins", &["a.md", "docs/b.md"], &[".markdownlint.json", "docs/.markdownlint.yaml"],
            &[".markdownlint.json", "docs/.markdownlint.yaml"]),
        ("files' order", &["docs/b.md", "a.md"], &[".markdownlint.json", "docs/.markdownlint.yaml"],
            &["docs/.markdownlint.yaml", ".markdownlint.json"]),
        ("inherited two levels", &["docs/api/x.md"], &[".markdownlint.json"], &[".markdownlint.json"]),
        ("a sibling's config", &["docs/b.md"], &["other/.markdownlint.json"], &[]),
        ("a config below the file", &["a.md"], &["docs/.markdownlint.json"], &[]),
    ];
    for (label, files, present, expected) in rows {
        let present: Vec<PathBuf> = present.iter().map(|path| Path::new(FOLDER).join(path)).collect();
        let is_file = |path: &Path| present.iter().any(|known| known == path);
        let files: Vec<PathBuf> = files.iter().map(PathBuf::from).collect();
        let expected: Vec<PathBuf> = expected.iter().map(PathBuf::from).collect();
        assert_eq!(applying_configs(Path::new(FOLDER), &files, &is_file), expected, "{label}");
    }
}

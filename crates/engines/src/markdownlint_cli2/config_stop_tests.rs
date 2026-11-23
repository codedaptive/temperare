//! ZUT for config_stop.rs with a fake filesystem: the options-file refusal first, then a check not in temperare's
//! list, then each applying config read, refused or found turning the rule off. A row gives the stop's kind and the
//! tokens its text must hold.

use super::config_stop;
use crate::types::Stop;
use std::io;
use std::path::{Path, PathBuf};

const FOLDER: &str = "/w/repo";

#[test]
fn config_stop_cases() {
    let json_off = (".markdownlint.json", "{\"MD013\": false}");
    let rows: [(&str, &str, &[&str], &[(&str, &str)], &str, &[&str]); 16] = [
        ("no config", "MD013", &["a.md"], &[], "none", &[]),
        ("options before the list", "MD999", &["a.md"], &[(".markdownlint-cli2.jsonc", "{}")], "failed", &[".markdownlint-cli2.jsonc"]),
        ("unlisted check", "MD999", &["a.md"], &[json_off], "not-listed", &["MD999", "markdownlint 0.42.2"]),
        ("a tag is not a check", "headings", &["a.md"], &[], "not-listed", &["headings"]),
        ("jsonc", "MD013", &["a.md"], &[(".markdownlint.jsonc", "{}")], "failed", &[".markdownlint.jsonc"]),
        ("cjs below", "MD013", &["docs/a.md"], &[("docs/.markdownlint.cjs", "")], "failed", &["docs/.markdownlint.cjs"]),
        ("unreadable", "MD013", &["a.md"], &[(".markdownlint.json", "UNREADABLE")], "failed", &[".markdownlint.json", "denied"]),
        ("repeated key", "MD013", &["a.md"], &[(".markdownlint.json", "{\"a\": 1, \"a\": 2}")], "failed", &[".markdownlint.json", "duplicate key"]),
        ("quoted falsy yaml", "MD013", &["a.md"], &[(".markdownlint.yaml", "MD013: \"false\"\n")], "failed", &[".markdownlint.yaml", "line 1"]),
        ("extends", "MD013", &["a.md"], &[(".markdownlint.json", "{\"extends\": \"base.json\"}")], "failed", &["`extends`"]),
        ("off in json", "MD013", &["a.md"], &[json_off], "not-available", &[".markdownlint.json", "`MD013`"]),
        ("off by alias in yaml", "MD013", &["a.md"], &[(".markdownlint.yaml", "line-length: false\n")], "not-available",
            &[".markdownlint.yaml", "`line-length`"]),
        ("check in another case", "Line-Length", &["a.md"], &[json_off], "not-available", &["`MD013`"]),
        ("on with options", "MD013", &["a.md"], &[(".markdownlint.json", "{\"MD013\": {\"line_length\": 200}}")], "none", &[]),
        ("off in any applying config", "MD013", &["a.md", "docs/b.md"],
            &[(".markdownlint.json", "{}"), ("docs/.markdownlint.yml", "default: false\n")], "not-available",
            &["docs/.markdownlint.yml", "`default`"]),
        ("a config no file reaches", "MD013", &["a.md"], &[("docs/.markdownlint.json", "{\"MD013\": false}")], "none", &[]),
    ];
    for (label, check, files, present, kind, tokens) in rows {
        let present: Vec<(PathBuf, &str)> = present.iter().map(|(path, text)| (Path::new(FOLDER).join(path), *text)).collect();
        let is_file = |path: &Path| present.iter().any(|(known, _)| known == path);
        let read = |path: &Path| match present.iter().find(|(known, _)| known == path) {
            Some((_, "UNREADABLE")) => Err(io::Error::new(io::ErrorKind::PermissionDenied, "permission denied")),
            Some((_, text)) => Ok(text.to_string()),
            None => Err(io::Error::new(io::ErrorKind::NotFound, "not found")),
        };
        let files: Vec<PathBuf> = files.iter().map(PathBuf::from).collect();
        let got = config_stop(check, Path::new(FOLDER), &files, &is_file, &read);
        let text = match (&got, kind) {
            (None, "none") => continue,
            (Some(Stop::Failed { code: None, stderr }), "failed") => stderr,
            (Some(Stop::NotListed { said }), "not-listed") => said,
            (Some(Stop::NotAvailable { why }), "not-available") => why,
            _ => panic!("{label}: expected {kind}, got {got:?}"),
        };
        for token in tokens {
            assert!(text.contains(token), "{label}: {token:?} not in {text:?}");
        }
    }
}

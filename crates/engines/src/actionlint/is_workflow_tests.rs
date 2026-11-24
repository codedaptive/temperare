//! ZUT for is_workflow.rs: a `.yml` or `.yaml` file directly in `.github/workflows` is a workflow; one elsewhere, one
//! a level deeper, or with another extension is not.

use super::is_workflow;
use std::path::Path;

#[test]
fn is_workflow_cases() {
    let rows = [
        ("yml", ".github/workflows/ci.yml", true),
        ("yaml", ".github/workflows/release.yaml", true),
        ("upper case extension", ".github/workflows/ci.YML", false),
        ("another extension", ".github/workflows/notes.md", false),
        ("no extension", ".github/workflows/yml", false),
        ("a level deeper", ".github/workflows/sub/ci.yml", false),
        ("the config", ".github/actionlint.yaml", false),
        ("at the root", "ci.yml", false),
        ("a nested .github", "sub/.github/workflows/ci.yml", false),
        ("a local action", ".github/actions/setup/action.yml", false),
    ];
    for (label, path, expected) in rows {
        assert_eq!(is_workflow(Path::new(path)), expected, "{label}");
    }
}

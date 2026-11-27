//! ZUT for options_suppression.rs: a key path holding `exclude` or a value `ignore`, in any file of the chain, is
//! Failed naming that file, the line and its path, in any YAML style; text cctao cannot read is Failed naming its
//! line; options without either are None.

use super::options_suppression;
use crate::types::Stop;
use std::path::PathBuf;

const BASE: &str = "/r/base.yaml";
const GOVERNING: &str = "/r/analysis_options.yaml";

#[test]
fn options_suppression_cases() {
    let clean = "include: package:lints/recommended.yaml\nanalyzer:\n  errors:\n    todo: warning\nlinter:\n  rules:\n    - prefer_single_quotes\n";
    let rows: [(&str, &str, &str, Option<&str>); 15] = [
        ("nothing hidden", "linter:\n  rules:\n    a: true\n", clean, None),
        ("errors ignore", "", "analyzer:\n  errors:\n    unused_local_variable: ignore\n",
            Some("/r/analysis_options.yaml: line 3: `analyzer: errors: unused_local_variable: ignore`:")),
        ("exclude", "", "analyzer:\n  exclude:\n    - gen/**\n",
            Some("/r/analysis_options.yaml: line 2: `analyzer: exclude`:")),
        ("in the included file", "analyzer:\n  exclude:\n    - build/**\n", clean,
            Some("/r/base.yaml: line 2: `analyzer: exclude`:")),
        ("quoted, upper case", "", "analyzer:\n  errors:\n    'dead_code': \"IGNORE\"\n",
            Some("/r/analysis_options.yaml: line 3: `analyzer: errors: dead_code: IGNORE`:")),
        ("ignore as a list item", "", "linter:\n  rules:\n    - ignore\n",
            Some("/r/analysis_options.yaml: line 3: `linter: rules: - ignore`:")),
        ("exclude key elsewhere, any case", "", "linter:\n  Exclude: x\n",
            Some("/r/analysis_options.yaml: line 2: `linter: Exclude: x`:")),
        ("escaped ignore (measured)", "", "analyzer:\n  errors:\n    unused_local_variable: \"\\x69gnore\"\n",
            Some("/r/analysis_options.yaml: line 3: `analyzer: errors: unused_local_variable: ignore`:")),
        ("quoted over two lines (measured)", "",
            "analyzer:\n  errors: {x: \"a\n    # b\", unused_local_variable: ignore}\n",
            Some("/r/analysis_options.yaml: line 3: `analyzer: errors: unused_local_variable: ignore`:")),
        ("flow style", "", "analyzer:\n  errors: {x: ignore}\n",
            Some("/r/analysis_options.yaml: line 2: `analyzer: errors: x: ignore`:")),
        ("value on the next line", "", "analyzer:\n  errors:\n    unused_local_variable:\n      ignore\n",
            Some("/r/analysis_options.yaml: line 4: `analyzer: errors: unused_local_variable: ignore`:")),
        ("indented unlike", "a:\n    b: 1\n  c: 2\n", clean, Some("/r/base.yaml: line 3: cctao cannot read it: ")),
        ("commented out", "", "analyzer:\n  errors:\n    # dead_code: ignore\n", None),
        ("part of a longer word", "", "linter:\n  rules:\n    - x_ignore\n    - ignore_for_file\n    - excludes\n", None),
        ("no options text", "", "", None),
    ];
    let mut failed = Vec::new();
    for (name, base, governing, want) in rows {
        let chain = [(PathBuf::from(BASE), base.to_string()), (PathBuf::from(GOVERNING), governing.to_string())];
        let got = options_suppression(&chain);
        let ok = match (&got, want) {
            (None, None) => true,
            (Some(Stop::Failed { code: None, stderr }), Some(head)) => {
                stderr.starts_with(head) && stderr.contains("rule file")
            }
            _ => false,
        };
        if !ok {
            eprintln!("{name}: got {got:?}");
            failed.push(name);
        }
    }
    assert!(failed.is_empty(), "failed rows: {failed:?}");
}

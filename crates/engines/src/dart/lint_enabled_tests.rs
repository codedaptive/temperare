//! ZUT for lint_enabled.rs: enabled only by an item under `linter: rules:` or `linter: rules: <lint>: true`, block or
//! flow, and never with any other value for `linter: rules: <lint>` in the chain, or a file YAML cannot read.

use super::lint_enabled;
use std::path::PathBuf;

#[test]
fn lint_enabled_cases() {
    let listed = "linter:\n  rules:\n    - prefer_single_quotes\n";
    let rows: [(&str, &str, &str, bool); 22] = [
        ("list item", "", listed, true),
        ("mapping true", "", "linter:\n  rules:\n    prefer_single_quotes: true\n", true),
        ("mapping TRUE", "", "linter:\n  rules:\n    prefer_single_quotes: TRUE\n", true),
        ("quoted item, comment", "", "linter:\n  rules:\n    - 'prefer_single_quotes' # style\n", true),
        ("enabled by the included file", listed, "include: base.yaml\n", true),
        ("enabled, with a severity", listed, "analyzer:\n  errors:\n    prefer_single_quotes: WARNING\n", true),
        ("no options text", "", "", false),
        ("another lint", "", "linter:\n  rules:\n    - avoid_print\n", false),
        ("only a severity", "", "analyzer:\n  errors:\n    prefer_single_quotes: warning\n", false),
        ("item under another key", "", "analyzer:\n  rules:\n    - prefer_single_quotes\n", false),
        ("commented out", "", "linter:\n  rules:\n    # - prefer_single_quotes\n", false),
        ("turned off by the governing file", listed, "linter:\n  rules:\n    prefer_single_quotes: false\n", false),
        ("quoted true, a string", "", "linter:\n  rules:\n    prefer_single_quotes: 'true'\n", false),
        ("tagged true", "", "linter:\n  rules:\n    prefer_single_quotes: !!str true\n", false),
        ("a bool tag on a word, file dropped by dart", listed, "x: !!bool maybe\n", false),
        ("listed twice, dropped by dart", "", "linter:\n  rules:\n    prefer_single_quotes: true\n    prefer_single_quotes: true\n", false),
        ("value not known", listed, "linter:\n  rules:\n    prefer_single_quotes: maybe\n", false),
        ("no value", listed, "linter:\n  rules:\n    prefer_single_quotes:\n", false),
        ("turned off on the next line", listed, "linter:\n  rules:\n    prefer_single_quotes:\n      false\n", false),
        ("enabled in flow style", "", "linter:\n  rules: [prefer_single_quotes]\n", true),
        ("escaped item", "", "linter:\n  rules:\n    - \"\\x70refer_single_quotes\"\n", true),
        ("another file unreadable", listed, "linter:\n\trules: {}\n", false),
    ];
    let mut failed = Vec::new();
    for (name, base, governing, want) in rows {
        let chain = [(PathBuf::from("/r/base.yaml"), base.to_string()),
            (PathBuf::from("/r/analysis_options.yaml"), governing.to_string())];
        if lint_enabled(&chain, "prefer_single_quotes") != want {
            failed.push(name);
        }
    }
    assert!(failed.is_empty(), "failed rows: {failed:?}");
}

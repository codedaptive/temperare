//! ZUT for config_suppression.rs: a key `ignore` (exact case, any style, through an anchor) is Failed naming the file,
//! the line and its key path; text `yaml_entries` cannot read (a duplicate key, a second document, a merge key) is
//! Failed naming its line; a config with neither is None.

use super::config_suppression;
use crate::types::Stop;
use std::path::Path;

const DROPS: &str = "actionlint drops findings matching it and tm cannot turn it off; put exceptions in the rule file";

#[test]
fn config_suppression_cases() {
    let file = "/r/.github/actionlint.yaml";
    let ignore = |at: &str| Some(format!("{file}: {at}: {DROPS}"));
    let unread = |at: &str| Some(format!("{file}: {at}; put exceptions in the rule file"));
    let rows: [(&str, &str, Option<String>); 10] = [
        ("plain ignore list", "paths:\n  \"**\":\n    ignore: ['undefined variable']\n", ignore("line 3: `paths: **: ignore`")),
        ("escaped key", "paths:\n  \"**\":\n    \"\\x69gnore\": [x]\n", ignore("line 3: `paths: **: ignore`")),
        ("ignore in an anchor aliased under paths", "base: &b\n  ignore: [x]\npaths:\n  \"**\": *b\n",
            ignore("line 2: `base: ignore`")),
        ("other case, not honoured", "paths:\n  \"**\":\n    Ignore: [x]\n", None),
        ("ignore as a value", "paths:\n  \"**\":\n    note: ignore\n", None),
        ("self-hosted-runner only", "self-hosted-runner:\n  labels: [my-runner]\nconfig-variables: [FOO]\n", None),
        ("empty", "", None),
        ("duplicate key", "paths:\n  a: {}\npaths:\n  b: {}\n",
            unread("line 3: tm cannot read it: duplicate mapping key `paths`")),
        ("second document", "self-hosted-runner:\n  labels: [a]\n---\npaths:\n  \"**\":\n    ignore: [x]\n",
            unread("line 3: tm cannot read it: a second YAML document")),
        ("merge key", "base: &b {ignore: [x]}\npaths:\n  \"**\": {<<: *b}\n",
            unread("line 3: tm cannot read it: a merge key `<<`")),
    ];
    for (label, text, expected) in rows {
        let expected = expected.map(|stderr| Stop::Failed { code: None, stderr });
        assert_eq!(config_suppression(Path::new(file), text), expected, "{label}");
    }
}

//! Unit tests for allowlist_key.rs: the path of the first allowlist key in each form and case gitleaks reads, an ancestor key
//! that is not a gitleaks table name hidden, and None where a config only mentions the word.

use super::allowlist_key;

#[test]
fn allowlist_key_finds_the_first_allowlist_key() {
    let secret = format!("AKIA{}", "Z7".repeat(8));
    let under_secret = format!("[\"{secret}\"]\nallowlist = {{}}\n");
    let dotted_secret = format!("\"{secret}.allowlist\" = {{}}\n");
    let rows: [(&str, &str, Option<&str>); 17] = [
        ("an ancestor key holding a secret", under_secret.as_str(), Some("<key>.allowlist")),
        ("a dotted key with a secret segment", dotted_secret.as_str(), Some("<key>.allowlist")),
        ("a quoted dotted key", "\"allowlist.paths\" = [\"x\"]\n", Some("allowlist")),
        ("a quoted dotted rule key", "\"rules.allowlist\" = {}\n", Some("rules.allowlist")),
        ("a table", "[allowlist]\npaths = [\"x\"]\n", Some("allowlist")),
        ("an array of tables", "[[allowlists]]\n", Some("allowlists")),
        ("a rule's allowlists", "[[rules]]\nid = \"r\"\n[[rules.allowlists]]\n", Some("rules[0].allowlists")),
        ("the second rule's allowlists", "[[rules]]\nid = \"a\"\n[[rules]]\nid = \"b\"\n[[rules.AllowLists]]\n", Some("rules[1].AllowLists")),
        ("an inline rule's allowlist", "rules = [{ id = \"r\", allowlist = { paths = [\"x\"] } }]\n", Some("rules[0].allowlist")),
        ("a mixed-case table", "[AllowList]\n", Some("AllowList")),
        ("an upper-case array of tables", "[[ALLOWLISTS]]\n", Some("ALLOWLISTS")),
        ("a \\u-escaped quoted key", "\"\\u0061llowlist\" = {}\n", Some("allowlist")),
        ("the word in a description", "description = \"no allowlist\"\n", None),
        ("the word as a regex", "regex = '''allowlist'''\n", None),
        ("the word as a path", "paths = [\"allowlist\"]\n", None),
        ("extend and rules only", "[extend]\nuseDefault = true\n[[rules]]\nid = \"r\"\nregex = '''x'''\n", None),
        ("an empty table", "", None),
    ];
    let failed: Vec<String> = rows
        .into_iter()
        .filter_map(|(name, toml, want)| {
            let got = toml.parse().map(|table| allowlist_key(&table)).map_err(|_| "does not parse");
            (got != Ok(want.map(str::to_string))).then(|| format!("{name}: got {got:?}").replace(&secret, "[redacted]"))
        })
        .collect();
    assert!(failed.is_empty(), "failed rows: {failed:#?}");
}

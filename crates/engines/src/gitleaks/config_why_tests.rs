//! Unit tests for config_why.rs: the parse why, the allowlist why with its key path, and the extend paths in each spelling.

use super::config_why;

#[test]
fn config_why_says_why_or_gives_the_extend_paths() {
    let rows: [(&str, &str, Result<Vec<&str>, &str>); 12] = [
        ("not TOML", "[allowlist\n", Err("gitleaks config `cfg.toml` does not parse as TOML, so tm cannot tell whether it declares an allowlist and reads no findings under it")),
        ("a plain allowlist", "[allowlist]\npaths = [\"x\"]\n", Err("gitleaks config `cfg.toml` declares an allowlist (`allowlist`); the rule file is the only home for an exception, so tm reads no findings under it")),
        ("a mixed-case rule allowlist", "[[rules]]\nid = \"r\"\n[[rules.AllowLists]]\n", Err("gitleaks config `cfg.toml` declares an allowlist (`rules[0].AllowLists`); the rule file is the only home for an exception, so tm reads no findings under it")),
        ("an extend path", "[extend]\npath = \"cfg/base.toml\"\n", Ok(vec!["cfg/base.toml"])),
        ("a mixed-case extend path", "[Extend]\nPath = \"x\"\n", Ok(vec!["x"])),
        ("an inline extend", "extend = { path = \"x\" }\n", Ok(vec!["x"])),
        ("a quoted dotted extend path", "\"extend.path\" = \"x\"\n", Ok(vec!["x"])),
        ("a quoted path key", "[extend]\n\"path\" = \"x\"\n", Ok(vec!["x"])),
        ("two case-variant extend tables", "[extend]\npath = \"b\"\n[Extend]\npath = \"a\"\n", Ok(vec!["a", "b"])),
        ("a path that is not a string", "[extend]\npath = 1\n", Ok(vec![])),
        ("extend without a path", "[extend]\nuseDefault = true\n", Ok(vec![])),
        ("empty text", "", Ok(vec![])),
    ];
    let failed: Vec<String> = rows
        .into_iter()
        .filter_map(|(name, text, want)| {
            let got = config_why("cfg.toml", text);
            (got.as_ref().map(|paths| paths.iter().map(String::as_str).collect::<Vec<_>>()).map_err(String::as_str) != want)
                .then(|| format!("{name}: got {got:?}"))
        })
        .collect();
    assert!(failed.is_empty(), "failed rows: {failed:#?}");
}

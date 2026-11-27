//! Unit test for eslint_unknown.rs: ESLint's not-defined line picked out of real stderr (fixtures/unknown_core.txt,
//! fixtures/unknown_plugin.txt; see fixtures/fixtures.txt) when it names the check, and None otherwise. The raw
//! `Could not find plugin` row is hand-built in the form the contract gives.

use super::eslint_unknown;
use procio::Output;

#[test]
fn eslint_unknown_finds_the_line_naming_the_check() {
    let core: &[u8] = include_bytes!("fixtures/unknown_core.txt");
    let plugin: &[u8] = include_bytes!("fixtures/unknown_plugin.txt");
    let raw: &[u8] = b"TypeError: Key \"rules\": Key \"react/jsx-key\": Could not find plugin \"react\" in configuration.\n";
    let rows: [(&str, &[u8], &str, Option<&str>); 7] = [
        ("unknown core rule", core, "no-such-rule-xyz", Some("TypeError: Key \"rules\": Key \"no-such-rule-xyz\": Could not find \"no-such-rule-xyz\" in plugin \"@\".")),
        ("plugin not loaded", plugin, "@typescript-eslint/no-explicit-any", Some("A configuration object specifies rule \"@typescript-eslint/no-explicit-any\", but could not find plugin \"@typescript-eslint\".")),
        ("raw plugin form", raw, "react/jsx-key", Some("TypeError: Key \"rules\": Key \"react/jsx-key\": Could not find plugin \"react\" in configuration.")),
        ("core line names another rule", core, "no-var", None),
        ("plugin line names another rule", plugin, "@typescript-eslint/no-unused-vars", None),
        ("check only starts the name", core, "no-such-rule", None),
        ("no words", b"Error: \"no-var\" crashed\n", "no-var", None),
    ];
    let failed: Vec<&str> = rows
        .iter()
        .filter(|(_, stderr, check, want)| {
            let out = Output { code: Some(2), stdout: Vec::new(), stderr: stderr.to_vec() };
            eslint_unknown(&out, check).as_deref() != *want
        })
        .map(|(name, ..)| *name)
        .collect();
    assert!(failed.is_empty(), "failed rows: {failed:?}");
}

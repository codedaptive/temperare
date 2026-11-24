//! ZUT for unknown_rule.rs: detekt's invalid-property line found when its path is exactly a `<set>><Rule>` pair the
//! check picks (unknown_rule.txt is detekt 1.23.8's real stdout for a config naming `style>NoSuchRule`;
//! fixtures/commands.txt), and nothing found for any other path or for other lines.

use super::unknown_rule;
use procio::Output;

const REAL: &[u8] = include_bytes!("fixtures/unknown_rule.txt");

#[test]
fn unknown_rule_cases() {
    let magic = "Property 'style>MagicNumber' is misspelled or does not exist.";
    let rows: [(&str, &[u8], &str, Option<&str>); 8] = [
        ("the real stdout, a rule the check does not pick", REAL, "MagicNumber", None),
        ("the picked pair", b"Property 'style>MagicNumber' is misspelled or does not exist.\n", "MagicNumber", Some(magic)),
        ("a pair the set check picks", b"Property 'style>MagicNumber' is misspelled or does not exist.\n", "style", Some(magic)),
        ("a typo under the set check's set", b"Property 'style>Typo' is misspelled or does not exist.\n", "style", None),
        ("an option under the picked rule", b"Property 'style>MagicNumber>bogusKey' is misspelled or does not exist.\n", "MagicNumber", None),
        ("the line indented", b"  Property 'style>MagicNumber' is misspelled or does not exist.  \n", "MagicNumber", Some(magic)),
        ("the line after a dash, as detekt repeats it", b"\t- Property 'style>MagicNumber' is misspelled or does not exist.\n", "MagicNumber", None),
        ("other words about the property", b"Property 'style>MagicNumber' is deprecated.\n", "MagicNumber", None),
    ];
    for (label, stderr, check, expected) in rows {
        let out = Output { code: Some(3), stdout: Vec::new(), stderr: stderr.to_vec() };
        assert_eq!(unknown_rule(&out, check).as_deref(), expected, "{label}");
    }
}

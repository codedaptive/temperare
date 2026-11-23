//! Unit tests for unknown_message.rs: pylint's W0012 and R0022 messages picked out of real json2 output (fixtures/, see
//! fixtures.txt) when they quote the rule's check, and left alone otherwise.

use super::unknown_message;
use procio::Output;

const UNKNOWN: &[u8] = include_bytes!("fixtures/unknown_id.json");
const REMOVED: &[u8] = include_bytes!("fixtures/removed_id.json");

#[test]
fn unknown_message_is_pylints_words_about_the_check() {
    let rows: [(&str, &[u8], &str, Option<&str>); 6] = [
        (
            "W0012 for ZZ9999",
            UNKNOWN,
            "ZZ9999",
            Some("Unknown option value for '--enable', expected a valid pylint message and got 'ZZ9999'"),
        ),
        (
            "R0022 for C0330",
            REMOVED,
            "C0330",
            Some("Useless option value for '--enable', 'C0330' was removed from pylint, see https://github.com/pylint-dev/pylint/pull/3571."),
        ),
        ("a W0012 naming another value", UNKNOWN, "C0121", None),
        ("a value the quoted one only starts with", UNKNOWN, "ZZ99", None),
        ("a finding that quotes the check is not W0012 or R0022", include_bytes!("fixtures/c0121.json"), "value == None", None),
        ("stdout that is not json2", b"No files to lint: exiting.\n", "ZZ9999", None),
    ];
    for (name, stdout, check, want) in rows {
        let out = Output { code: Some(0), stdout: stdout.to_vec(), stderr: Vec::new() };
        assert_eq!(unknown_message(&out, check), want.map(str::to_string), "{name}");
    }
}

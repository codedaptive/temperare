//! Unit test for detekt_output.rs: the report moved to stdout, the console text and stderr joined on stderr, the exit code
//! kept.

use super::detekt_output;
use procio::Output;

#[test]
fn detekt_output_cases() {
    let rows: [(&str, Output, Option<Vec<u8>>, Output); 3] = [
        (
            "findings with a report",
            Output { code: Some(2), stdout: b"Demo.kt:4:20: magic [MagicNumber]\n".to_vec(), stderr: b"warn\n".to_vec() },
            Some(b"{\"runs\":[]}".to_vec()),
            Output { code: Some(2), stdout: b"{\"runs\":[]}".to_vec(), stderr: b"Demo.kt:4:20: magic [MagicNumber]\nwarn\n".to_vec() },
        ),
        (
            "a config error and no report",
            Output { code: Some(3), stdout: b"Property 'style>X' is misspelled or does not exist.\n".to_vec(), stderr: Vec::new() },
            None,
            Output { code: Some(3), stdout: Vec::new(), stderr: b"Property 'style>X' is misspelled or does not exist.\n".to_vec() },
        ),
        (
            "killed, stderr only",
            Output { code: None, stdout: Vec::new(), stderr: b"Killed\n".to_vec() },
            None,
            Output { code: None, stdout: Vec::new(), stderr: b"Killed\n".to_vec() },
        ),
    ];
    for (name, out, sarif, want) in rows {
        assert_eq!(detekt_output(out, sarif), want, "{name}");
    }
}

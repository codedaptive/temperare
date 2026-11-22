//! ZUT for unknown_linters.rs: golangci-lint's unknown-linters line picked out of its stderr when it names a part of
//! the rule's check, and left alone otherwise. unknown_linters.stderr and unknown_two_linters.stderr are real
//! golangci-lint output (described in fixtures/how_made.txt).

use super::unknown_linters;
use procio::Output;

fn with_stderr(stderr: &[u8]) -> Output {
    Output { code: Some(3), stdout: Vec::new(), stderr: stderr.to_vec() }
}

#[test]
fn unknown_linters_finds_the_line_naming_a_part_of_the_check() {
    let line = "Error: unknown linters: 'nosuchlinter', run 'golangci-lint help linters' to see the list of supported \
                linters";
    let real: &[u8] = include_bytes!("fixtures/unknown_linters.stderr");
    let two: &[u8] = include_bytes!("fixtures/unknown_two_linters.stderr");
    let rows: Vec<(&str, &[u8], &str, Option<String>)> = vec![
        ("the real stderr, check nosuchlinter", real, "nosuchlinter", Some(line.to_string())),
        ("the real stderr, nosuchlinter in a list", real, "staticcheck,nosuchlinter", Some(line.to_string())),
        (
            "a part that is the second listed",
            two,
            "bar",
            Some(
                "Error: unknown linters: 'foo,bar', run 'golangci-lint help linters' to see the list of supported \
                 linters"
                    .to_string(),
            ),
        ),
        ("parts not in the quoted list", real, "staticcheck,govet", None),
        ("a part the listed linter only starts with", real, "nosuch", None),
        ("empty stderr", b"", "nosuchlinter", None),
    ];
    let wrong: Vec<String> = rows
        .into_iter()
        .filter_map(|(name, stderr, check, want)| {
            let got = unknown_linters(&with_stderr(stderr), check);
            (got != want).then(|| format!("{name}: got {got:?}, want {want:?}"))
        })
        .collect();
    assert!(wrong.is_empty(), "{wrong:#?}");
}

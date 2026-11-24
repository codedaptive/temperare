//! ZUT for integration_args.rs: `shellcheck` and `pyflakes` each turn on their own integration and need its program;
//! every other check turns both off.

use super::integration_args;

#[test]
fn integration_args_cases() {
    let rows: [(&str, Option<&str>, [&str; 2]); 4] = [
        ("shellcheck", Some("shellcheck"), ["-shellcheck=shellcheck", "-pyflakes="]),
        ("pyflakes", Some("pyflakes"), ["-shellcheck=", "-pyflakes=pyflakes"]),
        ("expression", None, ["-shellcheck=", "-pyflakes="]),
        ("shell-name", None, ["-shellcheck=", "-pyflakes="]),
    ];
    for (check, needs, args) in rows {
        assert_eq!(integration_args(check), (needs, args), "{check}");
    }
}

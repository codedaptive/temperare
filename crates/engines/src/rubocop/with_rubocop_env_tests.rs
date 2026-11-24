//! ZUT for with_rubocop_env.rs: the variables added, after the command's own, and nothing else changed.

use super::with_rubocop_env;
use procio::Cmd;
use std::ffi::OsString;
use std::path::PathBuf;

#[test]
fn with_rubocop_env_adds_the_empty_options_and_the_dead_proxy() {
    let cases: [(&str, Vec<(&str, &str)>); 2] = [
        ("no env of its own", vec![]),
        ("an env of its own", vec![("LANG", "C")]),
    ];
    let added = [
        ("RUBOCOP_OPTS", ""),
        ("http_proxy", "http://127.0.0.1:9"),
        ("https_proxy", "http://127.0.0.1:9"),
        ("HTTP_PROXY", "http://127.0.0.1:9"),
        ("HTTPS_PROXY", "http://127.0.0.1:9"),
        ("no_proxy", ""),
        ("NO_PROXY", ""),
    ];
    let mut failed = Vec::new();
    for (name, own) in cases {
        let result = std::panic::catch_unwind(|| {
            let pairs = |vars: &[(&str, &str)]| vars.iter().map(|(k, v)| (OsString::from(k), OsString::from(v))).collect::<Vec<_>>();
            let cmd = Cmd {
                args: vec![OsString::from("--version")],
                cwd: Some(PathBuf::from("/w/rb")),
                env: pairs(&own),
                ..Cmd::new("rubocop")
            };
            let got = with_rubocop_env(cmd);
            let mut want = pairs(&own);
            want.extend(pairs(&added));
            assert_eq!(got.env, want, "{name}");
            assert_eq!(got.program, OsString::from("rubocop"), "{name}");
            assert_eq!(got.args, [OsString::from("--version")], "{name}");
            assert_eq!(got.cwd, Some(PathBuf::from("/w/rb")), "{name}");
        });
        if result.is_err() { failed.push(name); }
    }
    assert!(failed.is_empty(), "failed rows: {failed:?}");
}

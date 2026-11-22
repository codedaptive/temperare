//! ZUT for clang_tidy_entry.rs: the fields of `CLANG_TIDY`, its version probe read on clang-tidy 23.1.6's
//! `--version` output, and its refusal, unknown-check and reader wired to clang-tidy's. Nothing runs.

use super::CLANG_TIDY;
use crate::entry::{Params, Pick};
use crate::types::Hit;
use procio::Output;
use rules::{Engine, RelPath};
use std::path::Path;

fn out(code: i32, stdout: &[u8], stderr: &[u8]) -> Output {
    Output { code: Some(code), stdout: stdout.to_vec(), stderr: stderr.to_vec() }
}

#[test]
fn clang_tidy_entry_fields() {
    assert_eq!((CLANG_TIDY.name, CLANG_TIDY.engine, CLANG_TIDY.program), ("clang-tidy", Engine::ClangTidy, "clang-tidy"));
    assert_eq!(CLANG_TIDY.needs, ["clang-tidy"]);
    assert_eq!(CLANG_TIDY.never, None);
    assert_eq!(CLANG_TIDY.args, ["--quiet", "--warnings-as-errors=-*"]);
    assert_eq!(CLANG_TIDY.pick, Pick::Arg("--checks=-*,"));
    assert!(matches!(CLANG_TIDY.params, Params::Args(args) if args.is_empty()), "{:?}", CLANG_TIDY.params);
    assert_eq!(CLANG_TIDY.config, Some("--config-file="));
    assert!(CLANG_TIDY.after.is_empty());

    let probe = CLANG_TIDY.version.unwrap();
    assert_eq!(probe.args, ["--version"]);
    let version = out(0, b"Homebrew LLVM version 23.1.6\n  Optimized build.\n", b"");
    assert_eq!((probe.read)(&version), Some("23.1.6".to_string()));

    let refusals: [(&str, bool); 2] = [("a,b", true), ("bugprone-branch-clone", false)];
    for (check, refused) in refusals {
        assert_eq!((CLANG_TIDY.refuse)(check).is_some(), refused, "{check}");
    }

    let unknown: [(&[u8], Option<&str>); 2] =
        [(b"Error: no checks enabled.\n", Some("Error: no checks enabled.")), (b"", None)];
    for (stderr, said) in unknown {
        assert_eq!((CLANG_TIDY.unknown)(&out(1, b"", stderr), "nosuch-check").as_deref(), said, "{stderr:?}");
    }

    let finding = out(0, b"/w/a.c:1:22: warning: statement should be inside braces [readability-braces-around-statements]\n", b"");
    let hit = Hit { path: RelPath::new("a.c"), line: Some(1), col: Some(22), what: "statement should be inside braces".to_string() };
    let read = (CLANG_TIDY.read)(&finding, "readability-braces-around-statements", Path::new("/w"), Path::new("/w"));
    assert_eq!(read, Ok(vec![hit]));
}

//! Unit tests for clippy_refusal: rustdoc lints cannot be checked by clippy.

use super::clippy_refusal::clippy_refusal;

#[test]
fn rustdoc_lints_are_refused_but_clippy_lints_and_warnings_are_not() {
    let cases = [
        ("rustdoc::broken_intra_doc_links", Some("clippy check `rustdoc::broken_intra_doc_links` is a rustdoc lint: clippy never runs rustdoc lints, so tm cannot report it")),
        ("clippy::bool_comparison", None),
        ("warnings", None),
    ];
    for (check, want) in cases {
        assert_eq!(clippy_refusal(check).as_deref(), want, "{check}");
    }
}

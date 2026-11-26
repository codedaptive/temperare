//! ZUT for hadolint_vars.rs: which names are kept as `HADOLINT_*` variables. Given names; no environment is read.

use super::hadolint_vars;
use std::ffi::OsString;

#[test]
fn hadolint_vars_picks_the_prefix() {
    let rows: [(&str, bool); 6] = [
        ("HADOLINT_IGNORE", true),
        ("HADOLINT_FOO", true),
        ("PATH", false),
        ("HADOLINT", false),
        ("hadolint_ignore", false),
        ("XHADOLINT_IGNORE", false),
    ];
    for (name, kept) in rows {
        let got = hadolint_vars([OsString::from(name)]);
        let want: Vec<OsString> = if kept { vec![OsString::from(name)] } else { vec![] };
        assert_eq!(got, want, "{name}: kept should be {kept}");
    }
    let all = hadolint_vars(rows.map(|(name, _)| OsString::from(name)));
    assert_eq!(all, ["HADOLINT_IGNORE", "HADOLINT_FOO"].map(OsString::from));
}

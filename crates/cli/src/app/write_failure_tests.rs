//! ZUT for write_failure.rs: the usage error on stderr, the `--json` object on stdout, and exit 2.

use super::write_failure::write_failure;
use crate::UsageError;

fn usage_error() -> UsageError {
    UsageError { what: "no rule file at /repo/tao.toml".to_string(), fix: "Write tao.toml.".to_string() }
}

#[test]
fn text_writes_the_error_on_stderr_only_and_returns_2() {
    let (mut out, mut err) = (Vec::new(), Vec::new());
    let code = write_failure(&usage_error(), false, &mut out, &mut err);
    assert_eq!(code, 2);
    assert_eq!(String::from_utf8(out).unwrap(), "");
    assert_eq!(
        String::from_utf8(err).unwrap(),
        "cctao: no rule file at /repo/tao.toml\nnext: Write tao.toml.\n"
    );
}

#[test]
fn json_also_writes_the_one_line_object_on_stdout() {
    let (mut out, mut err) = (Vec::new(), Vec::new());
    let code = write_failure(&usage_error(), true, &mut out, &mut err);
    assert_eq!(code, 2);
    assert_eq!(
        String::from_utf8(out).unwrap(),
        "{\"error\":\"no rule file at /repo/tao.toml\",\"exit\":2,\"next\":\"Write tao.toml.\"}\n"
    );
    assert_eq!(
        String::from_utf8(err).unwrap(),
        "cctao: no rule file at /repo/tao.toml\nnext: Write tao.toml.\n"
    );
}

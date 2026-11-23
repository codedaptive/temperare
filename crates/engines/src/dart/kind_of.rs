//! `kind_of`: what the probe run says a dart check is.

use super::CheckKind;
use crate::types::Stop;
use procio::Output;

/// The kind of `check` from `out`, the probe's run (`probe_files`: options that set `check` as an error code and
/// enable it as a lint, in a package with no Dart file), read as dart 3.14.1 prints it (fixtures/commands.txt): a
/// machine line whose code (the third field) is UNRECOGNIZED_ERROR_CODE means dart knows no such code, `UnknownCheck`
/// with dart's message (the eighth field); else an UNDEFINED_LINT line means a code that is not a lint, a
/// `Diagnostic`; else a `Lint`. The probe exits 0 with nothing printed for a lint and 2 for its warnings otherwise; any
/// other exit is `Failed` with its code and stderr, naming the check. Pure.
pub(crate) fn kind_of(out: &Output, check: &str) -> Result<CheckKind, Stop> {
    if !matches!(out.code, Some(0 | 2)) {
        let stderr = String::from_utf8_lossy(&out.stderr);
        return Err(Stop::Failed { code: out.code, stderr: format!("dart's probe of `{check}` failed: {stderr}") });
    }
    let stdout = String::from_utf8_lossy(&out.stdout);
    let said = |code: &str| {
        stdout.lines().find_map(|line| match line.splitn(8, '|').collect::<Vec<_>>()[..] {
            [_, _, found, _, _, _, _, message] if found == code => Some(message.to_string()),
            _ => None,
        })
    };
    if let Some(said) = said("UNRECOGNIZED_ERROR_CODE") {
        return Err(Stop::UnknownCheck { said });
    }
    Ok(if said("UNDEFINED_LINT").is_some() { CheckKind::Diagnostic } else { CheckKind::Lint })
}

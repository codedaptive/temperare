//! `not_listed`: whether cctao's list of hadolint codes holds a check (S-51).

use crate::read_code_list::read_code_list;
use crate::types::Stop;

/// What hadolint's SARIF log is missing for an SC code, said after the list's words. hadolint links ShellCheck but runs
/// it without its optional checks and drops some codes standalone shellcheck reports, so no list cctao could carry would
/// be true of the SC codes hadolint emits; a rule naming one is not available rather than a guess.
const NO_SC: &str = "hadolint runs ShellCheck without its optional checks and leaves out codes of its own, so cctao lists no SC code";

/// Ok when `check` is one of the codes `codes` (the text of cctao's list of hadolint codes) holds. hadolint never
/// says it does not know a code (`--error` takes any text), so a code not on the list is `NotListed`, never run and
/// never clean: its `said` is "`<check>` is not in cctao's list of hadolint codes (taken from hadolint
/// `<version>`)", and for a check starting `SC` that text then ": " and why no SC code is listed. A list
/// `read_code_list` cannot read is `Failed` naming it.
pub(crate) fn not_listed(codes: &str, check: &str) -> Result<(), Stop> {
    let list = read_code_list(codes).map_err(|e| Stop::Failed {
        code: None,
        stderr: format!("cctao's list of hadolint codes cannot be read: {e}"),
    })?;
    if list.codes.contains(&check) {
        return Ok(());
    }
    let said = format!("{check} is not in cctao's list of hadolint codes (taken from hadolint {})", list.version);
    Err(Stop::NotListed { said: if check.starts_with("SC") { format!("{said}: {NO_SC}") } else { said } })
}

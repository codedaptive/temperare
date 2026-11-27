//! `list_version`: the engine release cctao's list of an engine's codes was taken from. ShellCheck's and
//! hadolint's come from the `# version:` line of their code list files; detekt's is `DETEKT_LIST_VERSION`;
//! markdownlint's is written here.

use rules::Engine;

use crate::detekt::DETEKT_LIST_VERSION;
use crate::read_code_list::read_code_list;

/// The release of `engine` cctao's known-check list for it was taken from: the `# version:` line of ShellCheck's
/// `shellcheck_codes.txt` and of hadolint's `hadolint_codes.txt` (None if that file cannot be read as a code list),
/// `DETEKT_LIST_VERSION` for detekt's `detekt_rules.txt` and `detekt_typed_rules.txt`; None for an engine with no
/// such list. markdownlint's list is library markdownlint 0.42.2's (`MD_RULES_FROM`), but its probe reads the
/// markdownlint-cli2 version, so its release here is the markdownlint-cli2 release bundling that library
/// (`markdownlint_cli2/fixtures/fixtures.txt:5`).
pub(crate) fn list_version(engine: Engine) -> Option<&'static str> {
    match engine {
        Engine::ShellCheck => file_version(include_str!("../shellcheck/shellcheck_codes.txt")),
        Engine::Hadolint => file_version(include_str!("../hadolint/hadolint_codes.txt")),
        Engine::Detekt => Some(DETEKT_LIST_VERSION),
        Engine::Markdownlint => Some("0.24.2"),
        _ => None,
    }
}

/// The `# version:` value of a code list file's text, or None when the text does not read as a code list.
fn file_version(text: &'static str) -> Option<&'static str> {
    read_code_list(text).ok().map(|list| list.version)
}

//! `report_why`: why one file's result in stylelint's report is no answer for the check.

use super::FileReport;
use crate::rel_to_folder::rel_to_folder;
use std::path::Path;

/// Why `file`'s result is no answer, the first that applies: the config's `ignoreFiles` covers it (a hidden exception:
/// exceptions belong in the rule file), its config's `fix` rewrote it (tm never edits the code it checks), stylelint
/// could not parse it (a `CssSyntaxError` warning, else the first `parseErrors` entry), or the config gives a rule an
/// invalid option (the first `invalidOptionWarnings` entry). None for a file stylelint linted as configured. The file
/// is named relative to `folder` (`cwd` is where stylelint ran, both canonical), or by its absolute path when it is
/// outside `folder`.
pub(crate) fn report_why(file: &FileReport, cwd: &Path, folder: &Path) -> Option<String> {
    let name = || {
        rel_to_folder(cwd, folder, &file.source).map_or_else(|| file.source.clone(), |path| path.as_str().to_string())
    };
    if file.ignored {
        return Some(format!(
            "stylelint ignored {} through ignoreFiles in its config; exceptions go in the rule file",
            name()
        ));
    }
    if file.autofixed {
        return Some(format!(
            "stylelint rewrote {}: its config turns on fix, and tm never edits the code it checks",
            name()
        ));
    }
    let unparsed = file
        .warnings
        .iter()
        .find(|warning| warning.rule.as_deref() == Some("CssSyntaxError"))
        .or(file.parse_errors.first());
    if let Some(problem) = unparsed {
        return Some(format!("stylelint could not parse {}: {}", name(), problem.text));
    }
    file.invalid_option_warnings
        .first()
        .map(|problem| format!("stylelint's config is invalid for {}: {}", name(), problem.text))
}

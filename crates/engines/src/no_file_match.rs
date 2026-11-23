//! `no_file_match`: whether a rule's paths match none of the files git lists for its rule file's folder.

use rules::{RelPath, Rule};

/// Why a rule ran nothing when `no_file_match` says its paths match no file of the folder's listing.
pub(crate) const NO_FILE_MATCH_WHY: &str = "its paths match no file in the rule file's folder";

/// True when `rule` applies to none of `files`, relative to the rule file's folder: the tracked and
/// untracked-not-ignored files of it (`files_in_folder`), or those of them a flow has narrowed to (ESLint's linted
/// files that git lists). `Rule::applies` is the rule's compiled scope (its paths, its excludes, its
/// kits and outside kits), the same test `findings_from` puts each hit to, so a rule this says no to could give no
/// finding however its engine ran. An empty `files` matches nothing.
pub(crate) fn no_file_match(rule: &Rule, files: &[RelPath]) -> bool {
    !files.iter().any(|file| rule.applies(file))
}

//! `no_file_match`: whether a rule's paths match none of the files git lists for its rule file's folder.

use rules::{RelPath, Rule};

/// True when `rule` applies to none of `files`, the tracked and untracked-not-ignored files of the rule file's folder
/// relative to it (`files_in_folder`). `Rule::applies` is the rule's compiled scope (its paths, its excludes, its
/// kits and outside kits), the same test `findings_from` puts each hit to, so a rule this says no to could give no
/// finding however its engine ran. An empty `files` matches nothing.
pub(crate) fn no_file_match(rule: &Rule, files: &[RelPath]) -> bool {
    !files.iter().any(|file| rule.applies(file))
}

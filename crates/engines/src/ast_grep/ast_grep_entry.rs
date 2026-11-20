//! `AST_GREP`: the ast-grep engine as an entry.

use super::{hits_ast_grep, parse_ast_grep};
use crate::entry::{version_word, Entry, Params, Pick, VersionProbe};
use rules::Engine;

/// `ast-grep scan --rule <filled rule file> --json=stream` run in the rule file's folder: every match of the rule
/// file's rules under that folder, one JSON object per line, read by `parse_ast_grep` (every line is a match of the
/// rule file, so the reader does not use the check). The rule's check names its rule YAML, whose `{{name}}`s the
/// params fill; the flow is `hits_ast_grep`. A rule's config is not passed.
pub static AST_GREP: Entry = Entry {
    name: "ast-grep",
    engine: Engine::AstGrep,
    program: "ast-grep",
    needs: &["ast-grep"],
    never: None,
    args: &["scan"],
    pick: Pick::File("--rule"),
    params: Params::Template,
    config: None,
    after: &["--json=stream"],
    version: Some(VersionProbe { args: &["--version"], read: version_word }),
    refuse: |_| None,
    unknown: |_, _| None,
    read: |out, _check, cwd, folder| parse_ast_grep(out, cwd, folder),
    run: hits_ast_grep,
};

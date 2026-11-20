//! `AST_GREP`: the ast-grep engine as an entry.

use super::{hits_ast_grep, parse_ast_grep};
use crate::entry::{version_word, Entry, Params, Pick, VersionProbe};
use rules::Engine;

/// `ast-grep scan --rule <filled rule file> --json=stream` run in the rule file's folder: every match of the rule
/// file's rules under that folder, one JSON object per line, read by `parse_ast_grep` (every line is a match of the
/// rule file, so the reader does not use the check). The rule's check names its rule YAML, or its set carries it; the
/// params fill its `{{name}}`s; the flow is `hits_ast_grep`. A rule's config is not passed. Count mode is set by two
/// params, taken out before the fill: `count = "setup-vs-tests"` turns it on, and `shared_setup` (a list of globs
/// relative to the rule file's folder, which needs `count`) names the files exempt. In it the rule YAML must match each
/// file's root node and each test function, and each file the change adds lines to whose non-blank lines outside a
/// test function outnumber those inside one is reported at its first added line (`count_ast_grep`, which reads the
/// JSON with `parse_count_matches`); it needs the check's base, and is not available without one.
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

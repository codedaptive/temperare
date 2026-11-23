//! `CLANG_TIDY`: the clang-tidy engine as an entry.

use super::{clang_tidy_refusal, parse_clang_tidy, run_clang_tidy, unknown_check};
use crate::entry::{version_word, Entry, Params, Pick, VersionProbe};
use rules::Engine;

/// clang-tidy run by its own flow, `run_clang_tidy`, with the command `clang_tidy_cmd` builds:
/// `clang-tidy --quiet --warnings-as-errors=-* --checks=-*,<check> (--config-file=<config> | --config={}) <files>`,
/// run in the rule file's folder and read by `parse_clang_tidy`. Each C-family source in the rule's scope is handed
/// over, absolute (`c_files`, `is_c_source`). A source with no compile_commands.json or compile_flags.txt at or above
/// it makes the rule not available (`no_compile_database`): clang-tidy would otherwise check it without flags and
/// exit 0. clang-tidy saying "Error: no checks enabled." makes it `UnknownCheck` (`unknown_check`). A check that is
/// not one check name is refused before anything runs (`clang_tidy_refusal`). A repository `.clang-tidy` is read only
/// when the rule's config names it; otherwise `--config={}` keeps clang-tidy from reading one. A clang-tidy rule
/// takes no params. clang-tidy reads only local files and needs no environment set to stay local: nothing reaches
/// the network.
pub static CLANG_TIDY: Entry = Entry {
    name: "clang-tidy",
    engine: Engine::ClangTidy,
    program: "clang-tidy",
    needs: &["clang-tidy"],
    never: None,
    args: &["--quiet", "--warnings-as-errors=-*"],
    pick: Pick::Arg("--checks=-*,"),
    params: Params::Args(&[]),
    config: Some("--config-file="),
    after: &[],
    version: Some(VersionProbe { args: &["--version"], read: version_word }),
    refuse: clang_tidy_refusal,
    unknown: unknown_check,
    read: parse_clang_tidy,
    run: run_clang_tidy,
};

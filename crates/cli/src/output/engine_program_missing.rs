//! `engine_program_missing`: whether a not-available why says one of the engine's own programs is missing.

use rules::Engine;

/// True when `why` is `<program> not found on PATH` for a program in `engine`'s entry's `needs`. Those are the
/// programs engines' availability checks for, so installing the engine supplies them. Engines give the same why for
/// any program a run starts, helpers included (clippy's run starts `cargo`, shellcheck's stop starts `git`); a helper
/// is not in `needs`, and installing the engine does not fix it. An engine with no entry has no programs of its own.
pub(crate) fn engine_program_missing(engine: Engine, why: &str) -> bool {
    engines::entry_for(engine).is_some_and(|entry| {
        entry.needs.iter().any(|program| why.strip_suffix(" not found on PATH") == Some(*program))
    })
}

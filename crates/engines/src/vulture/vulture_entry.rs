//! `VULTURE`: the vulture engine as an entry.

use super::{parse_vulture, run_vulture, vulture_refusal};
use crate::entry::{version_word, Entry, Params, Pick, VersionProbe};
use rules::Engine;

/// `vulture --config=/dev/null .` run in the rule file's folder, read by `parse_vulture`. Every Python file under the
/// folder is scanned, as ruff and pylint scan `.`; findings outside the rule's scope are dropped later. Vulture does
/// not read .gitignore, so a virtualenv inside the folder is scanned unless the rule's paths leave it out.
///
/// The check is one vulture code (V101 to V107, V201: the codes vulture's own `# noqa` uses). Vulture has no option
/// that selects a kind and reports all of them, so the reader keeps the rule's; anything else is refused before
/// running (`vulture_refusal`). There is no confidence floor: vulture's confidence is fixed by kind (60 for
/// definitions, 90 for imports, 100 for unreachable code and unused arguments), so a floor could only drop whole
/// kinds the check already chooses.
///
/// `--config=/dev/null` keeps vulture from reading the folder's pyproject.toml, whose `[tool.vulture]` ignore_names,
/// ignore_decorators, exclude and min_confidence would hide findings outside the rule file, and whose verbose,
/// sort_by_size and make_whitelist would change what it prints. A rule's config never reaches vulture.
///
/// Suppressions vulture still honours, none of which tm can turn off: a `# noqa` comment on the item's line (bare, or
/// naming its V code, F401 or F841); a whitelist module the repository keeps (one made with `--make-whitelist`, say),
/// which is a .py file under the folder, so the `.` scan reads it and the names it uses count as used, as any code's
/// do; and vulture's own bundled whitelists (`vulture/whitelists/<module>_whitelist.py`, for stdlib modules such as
/// sys, logging and unittest), one of which vulture reads whenever the scanned code imports its module.
///
/// With no Python file under the folder vulture prints nothing and exits 0, so the rule reads clean, as ruff's does.
/// Exit 3 with findings is an answer, not an error. `run_vulture` sets PYTHONWARNINGS=ignore; nothing reaches the
/// network. Vulture takes no params: any param makes the rule `Failed` before running. Vulture rules never share a
/// run: `shares_with` has no arm for it.
pub static VULTURE: Entry = Entry {
    name: "vulture",
    engine: Engine::Vulture,
    program: "vulture",
    needs: &["vulture"],
    never: None,
    args: &["--config=/dev/null"],
    pick: Pick::None,
    params: Params::Args(&[]),
    config: None,
    after: &["."],
    version: Some(VersionProbe { args: &["--version"], read: version_word }),
    refuse: vulture_refusal,
    unknown: |_, _| None,
    read: parse_vulture,
    run: run_vulture,
};

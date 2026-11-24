//! `VALE`: the Vale engine as an entry.

use super::{parse_vale, run_vale, vale_refusal, vale_unknown};
use crate::entry::{version_word, Entry, Params, Pick, VersionProbe};
use rules::Engine;

/// `vale --no-global --output=JSON --filter=<vale_filter> --config=<wrapper> -- <files>`, run in the rule file's
/// folder by its own flow, `run_vale`, and read by `parse_vale`. Before it, the flow probes the repository's config
/// with `vale --no-global [--config=<the rule's config>] ls-config` in the same folder: the rule's config reaches Vale
/// only there, so `config` is None here. A config the rule names stands in for the repository's `.vale.ini` and
/// meets the same checks below; with it, no `.vale.ini` is needed.
///
/// The check is one Vale rule, `Style.Rule` (`Vale.Spelling`, S-72 (2)); any other form is `Failed` before anything
/// runs (`vale_refusal`). The filter keeps that rule alone and makes Vale fail with its own words when no loaded rule
/// has the name (`vale_filter`, `vale_unknown`): the rule is then not available, never clean. A repository with no
/// config Vale can find is not available, naming the config names; a config outside the repository, or one whose
/// `BasedOnStyles` never names the check's style, is not available too; a config Vale cannot load, or a config the
/// rule names that Vale cannot find, is `Failed` with Vale's message.
///
/// Vale is pointed at a wrapper config tm writes in a temporary folder and removes after the run (`vale_wrapper`):
/// the repository's `StylesPath` and `Vocab`, `MinAlertLevel = suggestion` and `[*] BasedOnStyles = <the check's
/// style>`. Only the rule's git-listed files in a format Vale reads are passed (`is_vale_source`); with none the rule
/// is `NoFiles`, settled before availability (`engine_listed`, in `engine_hits_with`'s step 1).
///
/// Exit codes: 0 with no error-level alert, 1 with one, 2 when Vale stops on an error. A check run writes that error
/// to stderr as a JSON object (its warnings, such as W101 for a comment grammar not installed, are plain lines there
/// and are not errors); the ls-config probe's errors are plain text on stderr. So tm reads stdout's JSON and the check
/// run's stderr error objects, not the exit code. Environment: tm sets nothing for
/// the check run, and the ls-config probe runs with `PWD` removed (`run_vale`); `--no-global` keeps a user's global
/// config out, `--config` overrides `VALE_CONFIG_PATH` on the check run, and
/// nothing is fetched (never `vale sync`): a style must already be on the repository's StylesPath, or be the built-in
/// `Vale`.
///
/// Suppressions: the repository's skip keys and section settings (`BlockIgnores`, `TokenIgnores`, `IgnoredScopes`,
/// `SkippedScopes`, `IgnoredClasses`, `Style.Rule = NO`) are never copied into the wrapper, so they are off
/// (S-67 (1)). Inline `<!-- vale off -->` and `<!-- vale Style.Rule = NO -->` comments still hide findings, since Vale
/// cannot be told to ignore them (S-67 (2)). The repository's `Vocab` stays on, a stated limit (S-81 (2)). Vale takes
/// no params, and its rules never share a run: `shares_with` has no arm for it.
pub static VALE: Entry = Entry {
    name: "vale",
    engine: Engine::Vale,
    program: "vale",
    needs: &["vale"],
    never: None,
    args: &["--no-global", "--output=JSON"],
    pick: Pick::None,
    params: Params::Args(&[]),
    config: None,
    after: &[],
    version: Some(VersionProbe { args: &["--version"], read: version_word }),
    refuse: vale_refusal,
    unknown: vale_unknown,
    read: parse_vale,
    run: run_vale,
};

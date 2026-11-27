//! `list_floor`: the why of a rule whose engine is older than the release cctao's list of its codes was taken from.

use super::list_version;
use crate::markdownlint_cli2::MD_RULES_FROM;
use crate::types::EngineVersion;
use rules::Engine;

/// The why when the first of `versions` for `engine` reads as `a.b.c` (three dot-separated non-negative integers)
/// and is lower than `list_version(engine)`, its three numbers compared left to right. None when `engine` has no
/// list, is not in `versions`, has no version, or has one that is equal, newer or does not read as `a.b.c`: the rule
/// then runs and is settled as without a floor. markdownlint's why names the markdownlint-cli2 release and the
/// library it bundles, since the probe reads the CLI and the list is the library's.
pub(crate) fn list_floor(engine: Engine, versions: &[EngineVersion]) -> Option<String> {
    let floor = list_version(engine)?;
    let installed = versions.iter().find(|v| v.engine == engine)?.version.as_deref()?;
    if triple(installed)? >= triple(floor)? {
        return None;
    }
    Some(match engine {
        Engine::Markdownlint => format!(
            "markdownlint-cli2 {installed} is older than markdownlint-cli2 {floor}, the release bundling \
             {MD_RULES_FROM}, which cctao's list of markdownlint codes was taken from"
        ),
        _ => {
            let name = engine.name();
            format!("{name} {installed} is older than {name} {floor}, the release cctao's list of {name} codes was taken from")
        }
    })
}

/// `version` as its three numeric parts when it is exactly `a.b.c` with each part all ASCII digits.
fn triple(version: &str) -> Option<(u64, u64, u64)> {
    let mut parts = version
        .split('.')
        .map(|part| Some(part).filter(|part| part.bytes().all(|b| b.is_ascii_digit())).and_then(|part| part.parse().ok()));
    let got = (parts.next()??, parts.next()??, parts.next()??);
    parts.next().is_none().then_some(got)
}

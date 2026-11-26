//! `resolve_base`: `tm check --base <rev>` turned into the repository top and the commit it names.

use super::CheckWorld;
use crate::UsageError;
use report::Oid;
use std::path::{Path, PathBuf};

/// With no base, `Ok(None)` and nothing is called. Otherwise the top of the repository holding `folder` and the
/// commit `rev` names there. A git failure, or a `rev` that names no commit, is a usage error: a base that does not
/// resolve never gives an empty report.
pub(crate) fn resolve_base(
    base: Option<&str>,
    folder: &Path,
    world: &CheckWorld,
) -> Result<Option<(PathBuf, Oid)>, UsageError> {
    let Some(rev) = base else {
        return Ok(None);
    };
    let top = (world.toplevel)(folder).map_err(|g| UsageError {
        what: format!("git could not find the repository top for --base: {g}"),
        fix: "Run tm check inside a git repository to use --base, or leave --base out.".to_string(),
    })?;
    match (world.rev_parse)(&top, rev) {
        Ok(Some(oid)) => Ok(Some((top, oid))),
        Ok(None) => Err(UsageError {
            what: format!("base '{rev}' does not name a commit"),
            fix: "Pass a commit, branch or tag that exists, e.g. --base main.".to_string(),
        }),
        Err(g) => Err(UsageError {
            what: format!("git could not resolve base '{rev}': {g}"),
            fix: format!("Check that git runs here and that {rev} exists."),
        }),
    }
}

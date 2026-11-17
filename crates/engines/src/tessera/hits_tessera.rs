//! `hits_tessera`: one tessera rule, which is never run.

use crate::host::Host;
use crate::types::{Hit, Stop};
use rules::Rule;
use std::path::Path;

/// Always `NotAvailable`: Tessera's slop records are not defined yet, so there is no output to read and `ts` is
/// neither run nor looked up (settled S-20). It takes the same arguments as every engine's run so the dispatch is
/// uniform.
pub(crate) fn hits_tessera(_rule: &Rule, _folder: &Path, _host: &Host) -> Result<Vec<Hit>, Stop> {
    Err(Stop::NotAvailable { why: "Tessera's slop records are not defined yet".to_string() })
}

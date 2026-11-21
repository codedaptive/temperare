//! `hits_tessera`: one tessera rule, which is never run.

use crate::entry::Entry;
use crate::host::Host;
use crate::types::{Hit, Stop};
use rules::Rule;
use std::path::Path;

/// Always `NotAvailable`: Tessera's slop records are not defined yet, so there is no output to read and `ts` is
/// neither run nor looked up. It is `TESSERA`'s flow, so it takes the arguments every flow takes.
pub(crate) fn hits_tessera(_entry: &Entry, _rule: &Rule, _folder: &Path, _host: &Host) -> Result<Vec<Hit>, Stop> {
    Err(Stop::NotAvailable { why: "Tessera's slop records are not defined yet".to_string() })
}

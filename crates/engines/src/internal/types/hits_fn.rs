//! `HitsFn`: one rule's flow, given the host its run is made with.

use crate::host::Host;
use crate::types::{Hit, Stop};
use rules::Rule;
use std::path::Path;

/// One rule's flow given its rule file's folder and a host: its hits, or why it stopped. Production passes
/// `engine_hits`; it is a seam so a test sees the host each rule's run is given (`run_rules_listed`, `run_group`).
pub(crate) type HitsFn = dyn Fn(&Rule, Result<&Path, &Stop>, &Host) -> Result<Vec<Hit>, Stop>;

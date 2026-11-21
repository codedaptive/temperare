//! `RunFn`: the shape of an engine's flow for one rule.

use super::Entry;
use crate::host::Host;
use crate::types::{Hit, Stop};
use rules::Rule;
use std::path::Path;

/// A flow: `(entry, rule, folder, host)` to the rule's hits, or why it stopped. `folder` is the rule file's folder
/// (canonical); every program runs through `host`, and the check run through `check_run` so it is counted. Crate-only,
/// as `Host` and `Stop` are.
pub(crate) type RunFn = fn(&Entry, &Rule, &Path, &Host) -> Result<Vec<Hit>, Stop>;

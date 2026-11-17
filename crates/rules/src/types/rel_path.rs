//! `RelPath`: a file path relative to the rule file's folder.

use serde::{Deserialize, Serialize};

/// A path relative to the rule file's folder, '/'-separated; `RelPath::new` normalises it (deserializing keeps the text as given).
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RelPath(pub(super) String);

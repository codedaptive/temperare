//! `Kit`: one `[kits.<name>]` of the rule file.

use crate::Scope;

/// A named set of paths and, optionally, the zone that owns them.
#[derive(Debug, Clone)]
pub struct Kit {
    pub name: String,
    /// The kit's path globs that compile.
    pub paths: Vec<String>,
    /// The kit's `owner` as written; None when absent, blank or not a string, since an owner is never blank.
    pub owner: Option<String>,
    /// The compiled `paths`; None when there are none, so the kit holds no file.
    pub scope: Option<Scope>,
}

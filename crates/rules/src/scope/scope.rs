//! `Scope`: which files a rule covers, by path globs, exclude globs and kits.

use globset::GlobSet;

/// The compiled globs of one rule. `include` None means every file; `kits` None means the rule names no kit.
#[derive(Debug, Clone)]
pub struct Scope {
    pub(super) include: Option<GlobSet>,
    pub(super) exclude: GlobSet,
    pub(super) kits: Option<GlobSet>,
}

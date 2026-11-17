//! `Scope::build`: compile a rule's globs.

use super::Scope;
use globset::{GlobBuilder, GlobSet, GlobSetBuilder};

impl Scope {
    /// Compiles `paths` (empty: every file), `exclude`, and `kit_paths` (None: no kits set; Some: the union of the
    /// listed kits' paths). Err describes the bad glob.
    pub fn build(paths: &[String], exclude: &[String], kit_paths: Option<&[String]>) -> Result<Scope, String> {
        Ok(Scope {
            include: if paths.is_empty() { None } else { Some(glob_set(paths)?) },
            exclude: glob_set(exclude)?,
            kits: kit_paths.map(glob_set).transpose()?,
        })
    }
}

/// One set of `globs`, where `*` never crosses '/' and `**` does.
fn glob_set(globs: &[String]) -> Result<GlobSet, String> {
    let mut set = GlobSetBuilder::new();
    for g in globs {
        let glob = GlobBuilder::new(g)
            .literal_separator(true)
            .build()
            .map_err(|e| format!("bad glob \"{g}\": {e}"))?;
        set.add(glob);
    }
    set.build().map_err(|e| format!("bad glob set: {e}"))
}

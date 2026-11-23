//! `ValeConfig`: what tm takes from the repository's Vale config, as `vale ls-config` resolved it. A value type with
//! derives only, so it has no unit-test sibling: a test of it could not fail; `ValeConfig::read`, which builds it, is
//! tested in vale_config_read_tests.rs.

use std::path::PathBuf;

/// The parts of the repository's resolved Vale config that tm reads (`ValeConfig::read`); nothing else of it reaches
/// the wrapper config.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ValeConfig {
    /// Every style some `BasedOnStyles` names: the `[*]` section's first, then each other section's in glob order,
    /// each named once.
    pub(crate) styles: Vec<String>,
    /// The `Vocab` names, in the config's order.
    pub(crate) vocab: Vec<String>,
    /// The resolved `StylesPath` folders, absolute; the wrapper names the first.
    pub(crate) paths: Vec<PathBuf>,
    /// Every config file Vale read: the root config, then the rest it loaded, each named once.
    pub(crate) files: Vec<PathBuf>,
}

//! `no_adapter_why`: why an engine with no adapter in this build is not available.

use rules::Engine;

/// The why for `engine` when it has no entry in this build (`entry_for` gives `None`), naming it by its rule-file
/// name.
pub(crate) fn no_adapter_why(engine: Engine) -> String {
    format!("no adapter for {} in this build", engine.name())
}

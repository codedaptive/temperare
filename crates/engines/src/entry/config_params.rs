//! `ConfigParams`: how an engine takes a rule's params through a temporary config file.

use crate::ParamError;
use serde_json::{Map, Value};

/// tm writes the text `text` makes from the params into `tm-<entry name>-<pid>-<n>.<ext>` under the temporary
/// folder, passes it as one argument `{option}{path}` and removes it after the run.
#[derive(Debug, Clone, Copy)]
pub struct ConfigParams {
    /// What the file's path is joined to, with its `=` (`"--config="`).
    pub option: &'static str,
    /// The file's extension, without the dot (`"toml"`).
    pub ext: &'static str,
    /// The file's text from the rule's params table; a param it cannot use is a `ParamError`, and the rule fails with
    /// it before anything runs.
    pub text: fn(&Map<String, Value>) -> Result<String, ParamError>,
}

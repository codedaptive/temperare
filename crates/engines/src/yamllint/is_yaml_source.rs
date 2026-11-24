//! `is_yaml_source`: whether a file is a YAML file yamllint is given.

use std::ffi::OsStr;
use std::path::Path;

/// True when `path`'s file name ends in `.yaml` or `.yml` (a file named only `.yaml` counts, as yamllint's `*.yaml`
/// matches it) or is exactly `.yamllint`: yamllint's stock `yaml-files` (`*.yaml`, `*.yml`, `.yamllint`), which match
/// the file name case-sensitively. The name is compared as bytes, so a name that is not UTF-8 is still judged.
pub(crate) fn is_yaml_source(path: &Path) -> bool {
    let Some(name) = path.file_name().map(OsStr::as_encoded_bytes) else {
        return false;
    };
    name.ends_with(b".yaml") || name.ends_with(b".yml") || name == b".yamllint"
}

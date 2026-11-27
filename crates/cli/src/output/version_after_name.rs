//! `version_after_name`: what `tm --version` prints after the name, shared with the telemetry line's version.

/// `<version> (<commit>)`: the workspace version and the commit the binary was built from (`unknown` when the build
/// could not read it).
pub(crate) fn version_after_name(version: &str, commit: &str) -> String {
    format!("{version} ({commit})")
}

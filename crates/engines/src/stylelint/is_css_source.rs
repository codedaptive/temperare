//! `is_css_source`: whether a file is a CSS file stylelint is given.

use std::path::Path;

/// True when `path`'s extension is `css` in any ASCII case: stylelint 18.1.0 linted an `a.CSS` and reported its finding
/// (fixtures/fixtures.txt). cctao gives stylelint these files only. stylelint does not refuse another syntax: with no
/// `customSyntax`, it read a `.scss` file holding a `$var` and nesting with no parse error and reported its finding
/// (fixtures/fixtures.txt), so the extension is what keeps other files out.
pub(crate) fn is_css_source(path: &Path) -> bool {
    path.extension().is_some_and(|ext| ext.eq_ignore_ascii_case("css"))
}

//! `is_css_source`: whether a file is a CSS file stylelint is given.

use std::path::Path;

/// True when `path`'s extension is `css` in any ASCII case (stylelint lints `a.CSS` as CSS too). Other syntaxes (SCSS,
/// Less, CSS in HTML or JS) need a `customSyntax` the repository installs, so tm gives stylelint `.css` files only.
pub(crate) fn is_css_source(path: &Path) -> bool {
    path.extension().is_some_and(|ext| ext.eq_ignore_ascii_case("css"))
}

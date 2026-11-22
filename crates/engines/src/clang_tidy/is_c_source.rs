//! `is_c_source`: whether a file is a C-family translation unit clang-tidy is given.

use std::path::Path;

/// The extensions of C, C++, Objective-C and Objective-C++ translation units.
const C_EXTENSIONS: [&str; 7] = ["c", "cc", "cpp", "cxx", "c++", "m", "mm"];

/// True when `path`'s extension is exactly one of `c`, `cc`, `cpp`, `cxx`, `c++`, `m` or `mm`. A header (`.h`,
/// `.hpp`) is false: its findings arrive through the translation units that include it.
pub(crate) fn is_c_source(path: &Path) -> bool {
    path.extension().is_some_and(|ext| C_EXTENSIONS.iter().any(|c| ext == *c))
}

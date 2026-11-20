//! `PACKAGE_PREFIXES`: where package managers put the programs they install.

/// The package-manager prefixes (Homebrew on Apple silicon, then `/usr/local`). A running binary under one belongs
/// to its package manager, so `temperare install` does not copy it.
pub const PACKAGE_PREFIXES: [&str; 2] = ["/opt/homebrew", "/usr/local"];

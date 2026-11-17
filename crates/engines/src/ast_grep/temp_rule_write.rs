//! `TempRule::write`: puts a filled rule YAML in a new temporary file.

use std::fs::OpenOptions;
use std::io::{self, Write};
use std::sync::atomic::{AtomicU64, Ordering};

use super::TempRule;

/// Numbers the files this process writes, so two rules written at once never share a name.
static NEXT: AtomicU64 = AtomicU64::new(0);

impl TempRule {
    /// A new file `tm-ast-grep-<pid>-<n>.yml` under `std::env::temp_dir()` holding `text`. It is created new, never
    /// opened over an existing file; when writing fails the file is removed again.
    pub fn write(text: &str) -> io::Result<TempRule> {
        let n = NEXT.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!("tm-ast-grep-{}-{n}.yml", std::process::id()));
        let mut file = OpenOptions::new().write(true).create_new(true).open(&path)?;
        let rule = TempRule { path };
        file.write_all(text.as_bytes())?;
        Ok(rule)
    }
}

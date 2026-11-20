//! `TempFile::write`: puts a text in a new temporary file.

use std::fs::OpenOptions;
use std::io::{self, Write};
use std::sync::atomic::{AtomicU64, Ordering};

use super::TempFile;

/// Numbers the files this process writes, so two files written at once never share a name.
static NEXT: AtomicU64 = AtomicU64::new(0);

impl TempFile {
    /// A new file `tm-<stem>-<pid>-<n>.<ext>` under `std::env::temp_dir()` holding `text`. It is created new, never
    /// opened over an existing file; when writing fails the file is removed again (the value is dropped).
    pub fn write(stem: &str, ext: &str, text: &str) -> io::Result<TempFile> {
        let n = NEXT.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!("tm-{stem}-{}-{n}.{ext}", std::process::id()));
        let mut file = OpenOptions::new().write(true).create_new(true).open(&path)?;
        let temp = TempFile { path };
        file.write_all(text.as_bytes())?;
        Ok(temp)
    }
}

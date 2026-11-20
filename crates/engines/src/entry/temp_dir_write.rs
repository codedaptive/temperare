//! `TempDir::write`: puts a text in a file named `file` in a new temporary folder.

use std::ffi::OsStr;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::path::{Component, Path};
use std::sync::atomic::{AtomicU64, Ordering};

use super::TempDir;

/// Numbers the folders this process makes, so two folders made at once never share a name.
static NEXT: AtomicU64 = AtomicU64::new(0);

impl TempDir {
    /// A new folder `tm-<stem>-<pid>-<n>` under `std::env::temp_dir()` holding the file `file` with `text`.
    /// `file` must be a plain file name, one path component (no root, no `.` or `..`, no separator, not empty), so
    /// the file is always inside the new folder; any other name is an `InvalidInput` error and nothing is made. The
    /// folder and the file are both created new, never over existing ones; when creating or writing the file fails
    /// the value is dropped, so the folder, and the file when this call created it, are removed again.
    pub fn write(stem: &str, file: &str, text: &str) -> io::Result<TempDir> {
        if !is_plain_name(file) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("the temporary file name `{file}` is not a plain file name"),
            ));
        }
        let n = NEXT.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!("tm-{stem}-{}-{n}", std::process::id()));
        std::fs::create_dir(&path)?;
        let mut temp = TempDir { file: None, path };
        let file = temp.path.join(file);
        let mut out = OpenOptions::new().write(true).create_new(true).open(&file)?;
        temp.file = Some(file);
        out.write_all(text.as_bytes())?;
        Ok(temp)
    }
}

/// `file`'s first component is a normal name equal to the whole of `file`, so it is exactly one such component.
fn is_plain_name(file: &str) -> bool {
    matches!(Path::new(file).components().next(), Some(Component::Normal(name)) if name == OsStr::new(file))
}

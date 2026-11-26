//! `exclude_arg`: vulture's `--exclude` argument that leaves out given folders and files, and nothing else.

use std::ffi::OsString;
use std::path::{Path, PathBuf};

/// The longest argument made. Linux takes at most 128 KiB in one argument (MAX_ARG_STRLEN); this keeps well under it.
const MAX_ARG_BYTES: usize = 98304;

/// `None` for no entries; else `--exclude=` and one pattern per `(path, is_folder)`, joined by `,`. Vulture splits the
/// value on `,` into fnmatch patterns, matches them against each module's absolute path, and wraps a pattern holding
/// none of `*?[` in `*…*`. So each `[`, `*` and `?` is escaped as a one-character class (`[[]`, `[*]`, `[?]`); a folder
/// becomes `<path>/*`, everything below it; and a file has its last character wrapped in a class (`/r/out.p[y]`), so
/// vulture never wraps it and the pattern matches that one path. On unix it works on the path's bytes, so a non-UTF-8
/// path survives; off unix, Err naming a path that is not UTF-8. Err, naming the path, for a path holding `,`, which
/// vulture would split; Err when the argument would pass `MAX_ARG_BYTES`.
pub(crate) fn exclude_arg(entries: &[(PathBuf, bool)]) -> Result<Option<OsString>, String> {
    if entries.is_empty() {
        return Ok(None);
    }
    let mut arg = b"--exclude=".to_vec();
    for (index, (path, is_folder)) in entries.iter().enumerate() {
        let bytes = path_bytes(path)?;
        if bytes.contains(&b',') {
            return Err(format!(
                "cannot leave the git-ignored path {} out of vulture's scan: vulture splits --exclude on commas",
                path.display()
            ));
        }
        if index > 0 {
            arg.push(b',');
        }
        if *is_folder {
            push_escaped(&mut arg, bytes);
            arg.extend_from_slice(b"/*");
        } else {
            let last = last_char_start(bytes);
            push_escaped(&mut arg, &bytes[..last]);
            arg.push(b'[');
            arg.extend_from_slice(&bytes[last..]);
            arg.push(b']');
        }
        if arg.len() > MAX_ARG_BYTES {
            return Err(format!(
                "too many git-ignored Python paths under the folder for vulture's --exclude (more than {MAX_ARG_BYTES} \
                 bytes in one argument); ignoring their folders instead (a `dir/` line in .gitignore) keeps the list short"
            ));
        }
    }
    Ok(Some(into_arg(arg)))
}

/// The path's bytes as the OS holds them.
#[cfg(unix)]
fn path_bytes(path: &Path) -> Result<&[u8], String> {
    Ok(path.as_os_str().as_encoded_bytes())
}

/// The path's UTF-8 bytes; Err naming a path that is not UTF-8.
#[cfg(not(unix))]
fn path_bytes(path: &Path) -> Result<&[u8], String> {
    path.to_str().map(str::as_bytes).ok_or_else(|| {
        format!(
            "cannot leave the git-ignored path {} out of vulture's scan: off unix, tm builds vulture's --exclude only \
             from UTF-8 paths",
            path.display()
        )
    })
}

/// The built bytes as the argument, unchanged.
#[cfg(unix)]
fn into_arg(arg: Vec<u8>) -> OsString {
    use std::os::unix::ffi::OsStringExt;
    OsString::from_vec(arg)
}

/// The built bytes as the argument. The lossy conversion never replaces a byte: every path is UTF-8 (`path_bytes`),
/// the bytes `exclude_arg` inserts are ASCII, and each cut falls at a character's start (`last_char_start`), so `arg`
/// is valid UTF-8.
#[cfg(not(unix))]
fn into_arg(arg: Vec<u8>) -> OsString {
    OsString::from(String::from_utf8_lossy(&arg).into_owned())
}

/// `bytes` with each fnmatch metacharacter as a one-character class. A `[`, `*` or `?` kept last by `exclude_arg` and
/// wrapped there gives the same class.
fn push_escaped(arg: &mut Vec<u8>, bytes: &[u8]) {
    for &byte in bytes {
        match byte {
            b'[' => arg.extend_from_slice(b"[[]"),
            b'*' => arg.extend_from_slice(b"[*]"),
            b'?' => arg.extend_from_slice(b"[?]"),
            _ => arg.push(byte),
        }
    }
}

/// Where the last character starts: the last byte that is not a UTF-8 continuation byte, so a multi-byte character
/// is wrapped whole.
fn last_char_start(bytes: &[u8]) -> usize {
    bytes.iter().rposition(|byte| byte & 0xC0 != 0x80).unwrap_or(0)
}

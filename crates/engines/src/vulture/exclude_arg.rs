//! `exclude_arg`: vulture's `--exclude` argument that leaves out given folders and files, and nothing else.

use std::ffi::OsString;
use std::os::unix::ffi::{OsStrExt, OsStringExt};
use std::path::PathBuf;

/// The longest argument made. Linux takes at most 128 KiB in one argument (MAX_ARG_STRLEN); this keeps well under it.
const MAX_ARG_BYTES: usize = 98304;

/// `None` for no entries; else `--exclude=` and one pattern per `(path, is_folder)`, joined by `,`. Vulture splits the
/// value on `,` into fnmatch patterns, matches them against each module's absolute path, and wraps a pattern holding
/// none of `*?[` in `*…*`. So each `[`, `*` and `?` is escaped as a one-character class (`[[]`, `[*]`, `[?]`); a folder
/// becomes `<path>/*`, everything below it; and a file has its last character wrapped in a class (`/r/out.p[y]`), so
/// vulture never wraps it and the pattern matches that one path. It works on the path's bytes, so a non-UTF-8 path
/// survives. Err, naming the path, for a path holding `,`, which vulture would split; Err when the argument would
/// pass `MAX_ARG_BYTES`.
pub(crate) fn exclude_arg(entries: &[(PathBuf, bool)]) -> Result<Option<OsString>, String> {
    if entries.is_empty() {
        return Ok(None);
    }
    let mut arg = b"--exclude=".to_vec();
    for (index, (path, is_folder)) in entries.iter().enumerate() {
        let bytes = path.as_os_str().as_bytes();
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
    Ok(Some(OsString::from_vec(arg)))
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

//! `names_to_try`: the file names `which_in` tries in each PATH folder, with Windows' PATHEXT rules or without.

use std::ffi::{OsStr, OsString};

/// The extensions used when PATHEXT lists none: the file kinds `run` can start on Windows. CreateProcess starts
/// `.exe` and Windows-program `.com` files (a 16-bit `.com` fails as `Spawn`), and Rust's `Command` starts `.bat` and
/// `.cmd` through its batch handling; `.vbs`, `.js` and the rest need a host program.
const DEFAULT_PATHEXT: [&str; 4] = [".COM", ".EXE", ".BAT", ".CMD"];

/// The file names to try for `name`, in the order they are tried.
///
/// `pathext` is `None` where there are no extension rules (unix): the name is tried as given. `Some` holds a PATHEXT
/// value, and the full file name is worked out here because Windows CreateProcess does not read PATHEXT: it appends
/// only `.exe`, and searches the working folder. Under PATHEXT rules a name holding `:` (drive-relative, like
/// `C:tool`) names no file in a PATH folder, so nothing is tried; a name already ending (ASCII case-insensitive) in a
/// listed extension is tried as given; any other gets each listed extension appended, in PATHEXT order, as written
/// there. An item is an extension when it starts with `.` and has more to it; the others (empty, `.` alone, no dot,
/// not UTF-8) are skipped, and a value with none left uses `DEFAULT_PATHEXT`.
pub(crate) fn names_to_try(name: &OsStr, pathext: Option<&OsStr>) -> Vec<OsString> {
    let Some(pathext) = pathext else {
        return vec![name.to_owned()];
    };
    let name_bytes = name.as_encoded_bytes();
    if name_bytes.contains(&b':') {
        return Vec::new();
    }
    let listed = extensions(pathext);
    let extensions = if listed.is_empty() { DEFAULT_PATHEXT.to_vec() } else { listed };
    if extensions.iter().any(|ext| ends_with_ignore_case(name_bytes, ext)) {
        return vec![name.to_owned()];
    }
    extensions
        .iter()
        .map(|ext| {
            let mut full = name.to_owned();
            full.push(ext);
            full
        })
        .collect()
}

/// The extensions a PATHEXT value lists, in order. `;` is ASCII, so splitting the encoded bytes on it leaves each
/// item whole.
fn extensions(pathext: &OsStr) -> Vec<&str> {
    pathext
        .as_encoded_bytes()
        .split(|byte| *byte == b';')
        .filter_map(|item| std::str::from_utf8(item).ok())
        .filter(|item| item.len() > 1 && item.starts_with('.'))
        .collect()
}

fn ends_with_ignore_case(name: &[u8], ext: &str) -> bool {
    name.len()
        .checked_sub(ext.len())
        .is_some_and(|start| name[start..].eq_ignore_ascii_case(ext.as_bytes()))
}

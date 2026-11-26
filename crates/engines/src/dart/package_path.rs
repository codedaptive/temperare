//! `package_path`: the file a `package:` URI names, through a package's `.dart_tool/package_config.json`.

use super::windows_path;
use serde_json::Value;
use std::path::{Path, PathBuf};

/// The file `uri` (`package:<name>/<path>`) names, as dart resolves it through `config_json`, the text of the
/// package_config.json in the folder `config_dir`: the entry of `packages` whose `name` is `<name>`, its `rootUri` (a
/// `file:///` URI, or a URI relative to `config_dir`), then its `packageUri` (`lib/` when absent), then `<path>`, each
/// percent-decoded. None when `uri` is not of that form, the JSON does not parse, no entry has the name, or a field
/// is missing, not a string, not decodable, or a URI of another scheme: the caller reads None as an include that
/// does not resolve. A `file://` URI with a host (UNC, `file://server/share`) is None on every platform. `windows`
/// is the platform, given as data (the caller passes `cfg!(windows)`): when set, the path is made native by
/// `windows_path`, so a file URL with a drive (`file:///C:/x`) gives `C:\x`, with `.` and `..` resolved lexically as
/// dart does, and a path with no drive, UNC included, is returned as built; when not set, it is returned as built.
/// Pure: it reads no file.
pub(crate) fn package_path(uri: &str, config_json: &str, config_dir: &Path, windows: bool) -> Option<PathBuf> {
    let (name, rest) = uri.strip_prefix("package:")?.split_once('/')?;
    let config: Value = serde_json::from_str(config_json).ok()?;
    let packages = config.get("packages")?.as_array()?;
    let package = packages.iter().find(|package| package.get("name").and_then(Value::as_str) == Some(name))?;
    let root_uri = package.get("rootUri")?.as_str()?;
    let root = match root_uri.strip_prefix("file://") {
        Some(path) if path.starts_with('/') => PathBuf::from(decoded(path)?),
        Some(_) => return None,
        None if root_uri.contains(':') => return None,
        None => config_dir.join(decoded(root_uri)?),
    };
    let package_uri = match package.get("packageUri") {
        None => "lib/",
        Some(value) => value.as_str()?,
    };
    let path = root.join(decoded(package_uri)?).join(decoded(rest)?);
    Some(match path.to_str() {
        Some(text) if windows => PathBuf::from(windows_path(text)),
        _ => path,
    })
}

/// `text` with each `%XX` escape decoded to its byte; None when an escape is cut short or either of its two bytes is
/// not a hex digit (checked first, since `from_str_radix` would take a sign: `%+f`), or the bytes are not UTF-8.
fn decoded(text: &str) -> Option<String> {
    let bytes = text.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut at = 0;
    while at < bytes.len() {
        if bytes[at] == b'%' {
            let hex = text.get(at + 1..at + 3).filter(|hex| hex.bytes().all(|byte| byte.is_ascii_hexdigit()))?;
            out.push(u8::from_str_radix(hex, 16).ok()?);
            at += 3;
        } else {
            out.push(bytes[at]);
            at += 1;
        }
    }
    String::from_utf8(out).ok()
}

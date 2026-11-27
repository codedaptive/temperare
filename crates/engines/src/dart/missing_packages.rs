//! `missing_packages`: a dart run that could not see the package's dependencies, because they were never fetched.

use crate::types::Stop;
use procio::Output;

/// The start of an INCLUDE_FILE_NOT_FOUND message, before the included URI.
const INCLUDE: &str = "|The URI '";
/// The start of a URI_DOES_NOT_EXIST message naming a package URI, before the rest of the URI.
const IMPORT: &str = "|Target of URI doesn't exist: 'package:";

/// `NotAvailable` when the package has no `.dart_tool/package_config.json` (`has_package_config` false) and `out`'s
/// stdout has an INCLUDE_FILE_NOT_FOUND line (an `include:` in analysis_options.yaml dart cannot find) or a
/// URI_DOES_NOT_EXIST line whose message names a `package:` URI: dart resolves `package:` URIs only through that
/// file, which `dart pub get` writes, so its findings would be those of a package without its dependencies. The why
/// names the first such URI and says to run `dart pub get` once, since cctao never fetches packages. `None` otherwise,
/// and always when the package config exists: a URI missing then is the code's own error, a diagnostic like any.
/// The code is the third field (the first three never hold an escape) and the URI is read from the message as dart
/// 3.14.1 words it, so a line worded otherwise is passed over; a URI holding `|` keeps dart's `\|` in the why.
pub(crate) fn missing_packages(out: &Output, has_package_config: bool) -> Option<Stop> {
    if has_package_config {
        return None;
    }
    let stdout = String::from_utf8_lossy(&out.stdout);
    let uri = stdout.lines().find_map(|line| {
        let [_, _, code, rest] = line.splitn(4, '|').collect::<Vec<_>>()[..] else {
            return None;
        };
        match code {
            "INCLUDE_FILE_NOT_FOUND" => quoted(rest, INCLUDE, ""),
            "URI_DOES_NOT_EXIST" => quoted(rest, IMPORT, "package:"),
            _ => None,
        }
    })?;
    Some(Stop::NotAvailable {
        why: format!(
            "dart cannot resolve `{uri}`: the package's dependencies are not fetched (no .dart_tool/package_config.json); \
             run `dart pub get` once in the package, since cctao never fetches packages"
        ),
    })
}

/// `prefix` and the text after `start` in `rest`, up to the next `'`; None when either is not there.
fn quoted(rest: &str, start: &str, prefix: &str) -> Option<String> {
    let after = &rest[rest.find(start)? + start.len()..];
    Some(format!("{prefix}{}", &after[..after.find('\'')?]))
}

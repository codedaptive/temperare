//! `detekt_args`: the arguments that give detekt its files, its config layers and the SARIF report's path.

use std::ffi::OsString;
use std::path::{Path, PathBuf};

/// The longest `--input` value made. Linux takes at most 128 KiB in one argument (MAX_ARG_STRLEN); this keeps well
/// under it, the same bound vulture's `--exclude` argument keeps (`exclude_arg`).
const MAX_ARG_BYTES: usize = 98304;

/// `--input <f1,f2,…>` (each file as listed, relative to the folder detekt runs in), then `--config <config>,<layer>`
/// (or `<layer>` alone when there is no config; detekt lets the last layer win, so cctao's layer goes last), then
/// `--report sarif:<report>`. Each option and its value are two arguments: detekt refuses `--config=<path>`. It works
/// on the paths' bytes, so a non-UTF-8 path survives. Err, naming the path, when a file, the config or the layer holds
/// `,` or `;`, since detekt splits those values on either; Err when the `--input` value would pass `MAX_ARG_BYTES`.
pub(crate) fn detekt_args(files: &[PathBuf], config: Option<&Path>, layer: &Path, report: &Path) -> Result<Vec<OsString>, String> {
    let input = joined(files.iter().map(PathBuf::as_path))?;
    if input.as_encoded_bytes().len() > MAX_ARG_BYTES {
        return Err(format!(
            "too many Kotlin files for detekt's --input (more than {MAX_ARG_BYTES} bytes in one argument); narrowing the \
             rule's paths keeps the list short"
        ));
    }
    let configs = joined(config.into_iter().chain([layer]))?;
    let mut sarif = OsString::from("sarif:");
    sarif.push(report);
    Ok(vec!["--input".into(), input, "--config".into(), configs, "--report".into(), sarif])
}

/// The paths' bytes joined by `,`; Err naming the first path that holds `,` or `;`.
fn joined<'a>(paths: impl Iterator<Item = &'a Path>) -> Result<OsString, String> {
    let mut value = OsString::new();
    for (index, path) in paths.enumerate() {
        if path.as_os_str().as_encoded_bytes().iter().any(|byte| matches!(byte, b',' | b';')) {
            return Err(format!("cannot give detekt the path {}: detekt splits its paths on `,` and `;`", path.display()));
        }
        if index > 0 {
            value.push(",");
        }
        value.push(path.as_os_str());
    }
    Ok(value)
}

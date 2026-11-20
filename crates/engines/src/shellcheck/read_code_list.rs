//! `read_code_list`: a code list's text read into a `CodeList`.

use super::CodeList;

/// `text`, line by line, each line trimmed: `# source: <text>` and `# version: <text>` give the list's source and
/// version (their values trimmed; the first of each counts); any other line starting with `#`, and a blank line, is
/// ignored; every other line is one code, in file order. Err naming the line missing, `no "# source:" line` or
/// `no "# version:" line`, when either is absent. A list with no codes is Ok.
pub(crate) fn read_code_list(text: &str) -> Result<CodeList<'_>, String> {
    let (mut source, mut version, mut codes) = (None, None, Vec::new());
    for line in text.lines().map(str::trim) {
        if let Some(value) = line.strip_prefix("# source:") {
            source = source.or(Some(value.trim()));
        } else if let Some(value) = line.strip_prefix("# version:") {
            version = version.or(Some(value.trim()));
        } else if !line.is_empty() && !line.starts_with('#') {
            codes.push(line);
        }
    }
    let source = source.ok_or_else(|| "no \"# source:\" line".to_string())?;
    let version = version.ok_or_else(|| "no \"# version:\" line".to_string())?;
    Ok(CodeList { source, version, codes })
}

//! `read_base_rule_file`: the rule file as it was at a `--base` revision.

use super::base_error::BaseError;
use super::RulesWorld;
use crate::UsageError;
use rules::RuleFile;
use std::path::{Path, PathBuf};

/// The rule file at `path` as committed at `rev`, read and checked with its folder. git is asked in that folder (an
/// empty parent is "."): `rev` must name a commit there, and the file is read by its file name, relative to that
/// folder, so the repository's top is never needed. ccvcslib refuses a file name beginning with `-` before it asks
/// git, so such a name is refused here rather than reported as missing. That name, a file name that is not UTF-8, a
/// revision that names no commit, a git failure, a missing file or bytes that are not UTF-8 are usage errors; the
/// file's problems come labelled `<path> (at <rev>)`.
pub(crate) fn read_base_rule_file(path: &Path, rev: &str, world: &RulesWorld) -> Result<RuleFile, BaseError> {
    let folder = match path.parent() {
        Some(parent) if !parent.as_os_str().is_empty() => parent,
        _ => Path::new("."),
    };
    let name = path.file_name().unwrap_or_default();
    let shown = name.to_string_lossy();
    let Some(name) = name.to_str() else {
        return Err(usage(
            format!("rule file name {shown} is not UTF-8"),
            "Rename the rule file to a UTF-8 name, or leave --loosened out.".to_string(),
        ));
    };
    if name.starts_with('-') {
        return Err(usage(
            format!("rule file name {shown} starts with '-', which cctao cannot read at a --base revision"),
            "Rename the rule file, or leave --loosened out.".to_string(),
        ));
    }
    let oid = match (world.rev_parse)(folder, rev) {
        Ok(Some(oid)) => oid,
        Ok(None) => {
            return Err(usage(
                format!("base '{rev}' does not name a commit"),
                "Pass a commit, branch or tag that exists, e.g. --base main.".to_string(),
            ))
        }
        Err(g) => {
            return Err(usage(
                format!("git could not resolve base '{rev}': {g}"),
                "Run cctao rules inside a git repository to use --base.".to_string(),
            ))
        }
    };
    let bytes = match (world.blob_at)(folder, &oid, name) {
        Ok(Some(bytes)) => bytes,
        Ok(None) => {
            return Err(usage(
                format!("no rule file {shown} at base '{rev}'"),
                format!("Pass a --base that has {shown}, or leave --loosened out."),
            ))
        }
        Err(g) => {
            return Err(usage(
                format!("git could not read {shown} at base '{rev}': {g}"),
                "Run cctao rules inside a git repository to use --base.".to_string(),
            ))
        }
    };
    let text = String::from_utf8(bytes).map_err(|_| {
        usage(
            format!("{shown} at base '{rev}' is not UTF-8 text"),
            format!("Pass a --base whose {shown} is UTF-8 text, or leave --loosened out."),
        )
    })?;
    RuleFile::parse(&text, folder)
        .map_err(|problems| BaseError::Problems(PathBuf::from(format!("{} (at {rev})", path.display())), problems))
}

/// A usage error as a `BaseError`.
fn usage(what: String, fix: String) -> BaseError {
    BaseError::Usage(UsageError { what, fix })
}

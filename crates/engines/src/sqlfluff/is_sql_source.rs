//! `is_sql_source`: whether a file is a SQL file sqlfluff is given.

use std::ffi::OsStr;
use std::path::Path;

/// sqlfluff 4.5.0's stock `sql_file_exts`. `x.sql.j2` has the extension `j2` to `Path::extension`, so a file is
/// matched on its name's suffix instead.
const SQL_SUFFIXES: [&str; 5] = [".sql", ".sql.j2", ".dml", ".ddl", ".pkb"];

/// True when `path`'s file name, lowercased (ASCII), ends with one of `SQL_SUFFIXES`, as sqlfluff matches a path it
/// is handed (`_match_file_extension` lowercases it), so `Q.SQL` is a source; false for a name that is not UTF-8.
pub(crate) fn is_sql_source(path: &Path) -> bool {
    path.file_name()
        .and_then(OsStr::to_str)
        .map(str::to_ascii_lowercase)
        .is_some_and(|name| SQL_SUFFIXES.iter().any(|suffix| name.ends_with(suffix)))
}

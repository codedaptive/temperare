//! ZUT for is_sql_source.rs: a file named with one of sqlfluff's stock SQL suffixes, in any case, is a source; any
//! other name is not.

use super::is_sql_source;
use std::path::Path;

#[test]
fn is_sql_source_cases() {
    let rows = [
        ("sql", "q.sql", true),
        ("sql in a folder", "db/q.sql", true),
        ("jinja sql", "models/q.sql.j2", true),
        ("dml", "q.dml", true),
        ("ddl", "q.ddl", true),
        ("pkb", "q.pkb", true),
        ("upper case", "Q.SQL", true),
        ("mixed case jinja", "q.Sql.J2", true),
        ("plain jinja", "q.j2", false),
        ("sql in the middle", "q.sql.bak", false),
        ("no extension", "sql", false),
        ("another language", "q.py", false),
    ];
    for (label, path, expected) in rows {
        assert_eq!(is_sql_source(Path::new(path)), expected, "{label}");
    }
}

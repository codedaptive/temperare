//! Unit test for is_vale_source.rs: every extension the formats run found Vale reading on its own is a source; plain-text
//! and outside-program formats, other cases, an unknown extension and none are not (fixtures/fixtures.txt, "Formats").

use super::is_vale_source;
use std::path::Path;

#[test]
fn is_vale_source_cases() {
    let sources = [
        "md", "markdown", "mdown", "mkd", "mkdn", "mdx", "rst", "adoc", "asciidoc", "asc", "html", "htm", "xhtml", "org",
        "txt", "text", "c", "h", "cc", "cpp", "hpp", "cs", "go", "java", "js", "jsx", "ts", "tsx", "kt", "kts", "php",
        "pl", "proto", "py", "r", "rb", "rs", "scala", "scss", "css", "less", "swift", "lua",
    ];
    let others = [
        "sh", "bash", "json", "yaml", "yml", "toml", "ini", "cfg", "sql", "tex", "rtf", "dart", "dita", "xml", "foo", "MD",
        "Py",
    ];
    let rows = sources.iter().map(|ext| (format!("docs/a.{ext}"), true)).chain(others.iter().map(|ext| (format!("a.{ext}"), false)));
    for (path, expected) in rows.chain([("Makefile".to_string(), false), ("md".to_string(), false), (".md".to_string(), false)]) {
        assert_eq!(is_vale_source(Path::new(&path)), expected, "{path}");
    }
}

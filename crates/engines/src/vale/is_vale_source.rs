//! `is_vale_source`: whether a file is in a format tm gives Vale.

use std::path::Path;

/// The extensions, matched as written, of the files Vale 3.28.1 reads on its own (no outside program, no grammar
/// fetched), from a run over one file of each (fixtures/fixtures.txt, "Formats"). Markup, whose prose Vale reads: md
/// markdown mdown mkd mkdn mdx rst adoc asciidoc asc html htm xhtml org txt text. Code, of which Vale reads only the
/// comments: the rest. Left out: what Vale reads as plain text, whole lines of code or data included (sh bash json
/// yaml yml toml ini cfg sql tex rtf dart, an unknown extension, none), which would report words that are not prose;
/// and what needs an outside program (dita, xml), which Vale answers with an error object. An extension in another
/// case is plain text too (fixtures/fixtures.txt, "Extension case"), so none is listed.
const VALE_EXTENSIONS: [&str; 43] = [
    "md", "markdown", "mdown", "mkd", "mkdn", "mdx", "rst", "adoc", "asciidoc", "asc", "html", "htm", "xhtml", "org",
    "txt", "text", "c", "h", "cc", "cpp", "hpp", "cs", "go", "java", "js", "jsx", "ts", "tsx", "kt", "kts", "php", "pl",
    "proto", "py", "r", "rb", "rs", "scala", "scss", "css", "less", "swift", "lua",
];

/// True when `path`'s extension is one of `VALE_EXTENSIONS`, case as written, as Vale matches it
/// (fixtures/fixtures.txt, "Extension case").
pub(crate) fn is_vale_source(path: &Path) -> bool {
    path.extension().and_then(|ext| ext.to_str()).is_some_and(|ext| VALE_EXTENSIONS.contains(&ext))
}

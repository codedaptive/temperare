//! `vale_errors`: the error objects Vale wrote to stderr during a check run.

use serde::Deserialize;

/// One error object as Vale 3.28.1 prints it with `--output=JSON`; its `Line`, `Path` and `Span` are not read.
#[derive(Deserialize)]
struct ErrorObject {
    #[serde(rename = "Code")]
    code: String,
    #[serde(rename = "Text")]
    text: String,
}

/// Each error object in `stderr`, in order, as its `(Code, Text)`. Vale writes them to stderr, not stdout, as
/// indented JSON whose first line is a lone `{`, after its human `W101` warning line (fixtures/dita.txt, one line and
/// one object). The reader takes every line that is exactly `{` as the start of one object, read up to its closing
/// brace, so it reads several; a line that starts no object is passed over. A run with no error object gives an empty list.
pub(crate) fn vale_errors(stderr: &[u8]) -> Vec<(String, String)> {
    let text = String::from_utf8_lossy(stderr);
    std::iter::once(0)
        .chain(text.match_indices('\n').map(|(at, _)| at + 1))
        .filter(|&at| text[at..].lines().next() == Some("{"))
        .filter_map(|at| serde_json::Deserializer::from_str(&text[at..]).into_iter::<ErrorObject>().next()?.ok())
        .map(|error| (error.code, error.text))
        .collect()
}

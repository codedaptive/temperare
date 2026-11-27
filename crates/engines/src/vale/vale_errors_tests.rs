//! Unit test for vale_errors.rs: the error objects in Vale 3.28.1's real stderr (fixtures/fixtures.txt says how each was
//! captured), one or none, past its W101 warning lines; human error text holds none. The "two objects" row is
//! hand-built, not a capture.

use super::vale_errors;

#[test]
fn vale_errors_cases() {
    let unknown_text = "invalid operation: int(unknown) (1:86)\n | filter(Rules, {.Name == \"Vale.Speling\" || (none(Rules, {.Name == \"Vale.Speling\"}) && int(\"unknown\") == 0)})\n | .....................................................................................^";
    let rows: [(&str, &[u8], Vec<(&str, &str)>); 6] = [
        ("unknown rule", include_bytes!("fixtures/unknown.txt"), vec![("E100", unknown_text)]),
        ("after a W101 line", include_bytes!("fixtures/dita.txt"), vec![("E100", "dita not found; it's needed to read DITA")]),
        (
            "style not on the StylesPath",
            include_bytes!("fixtures/missing_style.txt"),
            vec![("E201", "Style 'Gone' isn't on the StylesPath (/private/tmp/tm-vale-capture/gone/styles).")],
        ),
        ("human error text", include_bytes!("fixtures/load_error.txt"), vec![]),
        ("empty", b"", vec![]),
        (
            "two objects",
            b"{\n  \"Line\": 0,\n  \"Path\": \"\",\n  \"Text\": \"a\",\n  \"Code\": \"E100\",\n  \"Span\": 0\n}\n{\n  \"Line\": 0,\n  \"Path\": \"\",\n  \"Text\": \"b\",\n  \"Code\": \"E100\",\n  \"Span\": 0\n}\n",
            vec![("E100", "a"), ("E100", "b")],
        ),
    ];
    for (label, stderr, expected) in rows {
        let expected: Vec<(String, String)> = expected.into_iter().map(|(code, text)| (code.into(), text.into())).collect();
        assert_eq!(vale_errors(stderr), expected, "{label}");
    }
}

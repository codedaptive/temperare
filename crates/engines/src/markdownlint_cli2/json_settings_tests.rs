//! Unit test for json_settings.rs: top-level keys in file order with markdownlint's on/off for each value (JavaScript
//! truthiness; `default` by truthiness alone; an object by its `enabled`), and the texts it refuses. An `Err` row gives
//! a piece of the message.

use super::json_settings;

#[test]
fn json_settings_cases() {
    let rows: [(&str, &str, Result<Vec<(&str, bool)>, &str>); 22] = [
        ("empty object", "{}", Ok(vec![])),
        ("name and alias", r#"{"MD013": false, "line-length": true}"#, Ok(vec![("MD013", false), ("line-length", true)])),
        ("file order kept", r#"{"b": 1, "a": 0}"#, Ok(vec![("b", true), ("a", false)])),
        ("null", r#"{"MD013": null}"#, Ok(vec![("MD013", false)])),
        ("zero", r#"{"MD013": 0}"#, Ok(vec![("MD013", false)])),
        ("minus zero", r#"{"MD013": -0}"#, Ok(vec![("MD013", false)])),
        ("zero float", r#"{"MD013": 0.0e3}"#, Ok(vec![("MD013", false)])),
        ("empty string", r#"{"MD013": ""}"#, Ok(vec![("MD013", false)])),
        ("truthy values", r#"{"a": "warning", "b": 2, "c": [], "d": 1.5}"#,
            Ok(vec![("a", true), ("b", true), ("c", true), ("d", true)])),
        ("object without enabled", r#"{"MD013": {"line_length": 100}}"#, Ok(vec![("MD013", true)])),
        ("enabled false", r#"{"MD013": {"enabled": false, "line_length": 100}}"#, Ok(vec![("MD013", false)])),
        ("enabled falsy", r#"{"a": {"enabled": 0}, "b": {"enabled": null}, "c": {"enabled": ""}}"#,
            Ok(vec![("a", false), ("b", false), ("c", false)])),
        ("enabled truthy", r#"{"a": {"enabled": "x"}, "b": {"enabled": {}}, "c": {"Enabled": false}}"#,
            Ok(vec![("a", true), ("b", true), ("c", true)])),
        ("default object is truthy", r#"{"Default": {"enabled": false}}"#, Ok(vec![("Default", true)])),
        ("keys differing in case", r#"{"MD013": true, "md013": false}"#, Ok(vec![("MD013", true), ("md013", false)])),
        ("repeat deeper is not read", r#"{"MD044": {"names": {"a": 1, "a": 2}}}"#, Ok(vec![("MD044", true)])),
        ("repeated top-level key", r#"{"MD013": true, "MD013": false}"#, Err("duplicate key `MD013`")),
        ("repeated enabled", r#"{"MD013": {"enabled": true, "enabled": false}}"#, Err("duplicate key `enabled`")),
        ("array", "[]", Err("a JSON object")),
        ("JSONC comment", "{\"MD013\": false // off\n}", Err("line 1")),
        ("trailing comma", r#"{"MD013": false,}"#, Err("line 1")),
        ("text after", "{} x", Err("trailing characters")),
    ];
    for (label, text, expected) in rows {
        let got = json_settings(text);
        match expected {
            Ok(settings) => {
                let settings: Vec<(String, bool)> = settings.into_iter().map(|(key, on)| (key.to_string(), on)).collect();
                assert_eq!(got, Ok(settings), "{label}");
            }
            Err(piece) => assert!(got.as_ref().is_err_and(|err| err.contains(piece)), "{label}: {got:?}"),
        }
    }
}

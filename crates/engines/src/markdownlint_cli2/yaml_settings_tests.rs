//! Unit test for yaml_settings.rs: top-level keys in file order with markdownlint's on/off for each value, read as js-yaml
//! 5.5.0 reads YAML (the falsy and truthy spellings measured on it, `node` over its `load`), and the texts it refuses.
//! An `Err` row gives a piece of the message.

use super::yaml_settings;

#[test]
fn yaml_settings_cases() {
    let falsy = "a: false\nb: FALSE\nc: ~\nd:\ne: NULL\nf: 0\ng: -0\nh: +0\ni: 00\nj: 0.0\nk: .0\nl: 0.\nm: 0E5\nn: 0x00\no: 0o0\np: .NaN\n";
    let truthy = "a: yes\nb: no\nc: off\nd: 0b0\ne: -0x0\nf: .Nan\ng: fAlse\nh: 1\ni: .inf\nj: 0.1\nk: 1e0\nl: 0e\nm: 0_0\n";
    let all = |keys: &str, on: bool| keys.split(' ').map(|key| (key.to_string(), on)).collect::<Vec<_>>();
    let rows: [(&str, &str, Result<Vec<(String, bool)>, &str>); 30] = [
        ("underflow", "MD013: 1e-400\n", Ok(all("MD013", false))),
        ("zero exponent", "MD013: 0e5\n", Ok(all("MD013", false))),
        ("nonzero exponent", "MD013: 1e-5\n", Ok(all("MD013", true))),
        ("empty", "", Ok(vec![])),
        ("comment and marker only", "# nothing\n---\n", Ok(vec![])),
        ("name and alias", "MD013: false\nline-length: true\n", Ok(all("MD013", false).into_iter().chain(all("line-length", true)).collect())),
        ("falsy plain spellings", falsy, Ok(all("a b c d e f g h i j k l m n o p", false))),
        ("truthy plain spellings", truthy, Ok(all("a b c d e f g h i j k l m", true))),
        ("quoted", "a: \"\"\nb: \"warning\"\nc: 'true'\n", Ok(vec![("a".into(), false), ("b".into(), true), ("c".into(), true)])),
        ("mapping without enabled", "MD013:\n  line_length: 100\n", Ok(all("MD013", true))),
        ("enabled false", "MD013:\n  enabled: false\n  line_length: 100\n", Ok(all("MD013", false))),
        ("enabled no is a string", "MD013:\n  enabled: no\n", Ok(all("MD013", true))),
        ("enabled a mapping", "MD013:\n  enabled: {}\n", Ok(all("MD013", true))),
        ("enabled deeper is not read", "MD013:\n  x:\n    enabled: false\n", Ok(all("MD013", true))),
        ("sequence", "MD013: [ {enabled: false} ]\n", Ok(all("MD013", true))),
        ("default mapping is truthy", "default:\n  enabled: false\n", Ok(all("default", true))),
        ("next key after a mapping", "MD013:\n  a: 1\nMD009:\n  enabled: false\n",
            Ok(all("MD013", true).into_iter().chain(all("MD009", false)).collect())),
        ("alias replayed", "base: &b\n  enabled: false\nMD013: *b\n", Ok(all("base MD013", false))),
        ("plain JSON", "{\"MD013\": false}\n", Ok(all("MD013", false))),
        ("quoted false", "MD013: \"false\"\n", Err("line 1")),
        ("tagged false", "MD013: !!bool false\n", Err("line 1")),
        ("quoted zero", "MD013: '0'\n", Err("line 1")),
        ("quoted null", "MD013: \"null\"\n", Err("line 1")),
        ("quoted enabled false", "MD013:\n  enabled: 'false'\n", Err("line 2")),
        ("repeated key", "MD013: false\nMD013: true\n", Err("line 2")),
        ("TOML", "default = false\nMD013 = false\n", Err("not a mapping")),
        ("sequence at the top", "- MD013\n", Err("not a mapping")),
        ("JSONC comment", "{\"MD013\": /* x */ false}\n", Err("JSONC")),
        ("flow YAML that is not JSON", "{MD013: false}\n", Err("JSONC")),
        ("key holding =", "a = b: c\n", Err("holds `=`")),
    ];
    for (label, text, expected) in rows {
        let got = yaml_settings(text);
        match expected {
            Ok(settings) => assert_eq!(got, Ok(settings), "{label}"),
            Err(piece) => assert!(got.as_ref().is_err_and(|err| err.contains(piece)), "{label}: {got:?}"),
        }
    }
}

//! ZUT for yaml_entries.rs: each mapping value and sequence item with its key path, item flag, line and value, block
//! and flow style alike, escapes decoded, tagged scalars quoted, aliases (of nodes and of keys) replayed; the forms it
//! does not read, a repeated key, a core tag its text does not fit, a `!!` tag cctao does not read, entries multiplied
//! by aliases past the bound and nesting past the bound, an error naming the line.

use super::YamlValue::{self, Nested, Null};
use super::{yaml_entries, YamlEntry};

fn e(path: &[&str], item: bool, line: usize, value: YamlValue) -> YamlEntry {
    YamlEntry { path: path.iter().map(|key| key.to_string()).collect(), item, line, value }
}

fn p(text: &str) -> YamlValue {
    YamlValue::Plain(text.to_string())
}

fn q(text: &str) -> YamlValue {
    YamlValue::Quoted(text.to_string())
}

#[test]
fn yaml_entries_cases() {
    // Each line holds ten aliases of the one above: 11, 111, 1111, 11111 entries, then 111111 on line 5.
    let tens = |alias: &str| [alias; 10].join(", ");
    let bomb = format!("a: &a [{}]\nb: &b [{}]\nc: &c [{}]\nd: &d [{}]\ne: [{}]\n", tens("x"), tens("*a"), tens("*b"),
        tens("*c"), tens("*d"));
    // The root sequence and the 63 inside it are 64 levels, the most read; one more is past the bound.
    let sixty_four = format!("{}{}\n", "[".repeat(64), "]".repeat(64));
    let sixty_five = format!("{}{}\n", "[".repeat(65), "]".repeat(65));
    let rows: Vec<(&str, &str, Result<Vec<YamlEntry>, (usize, &str)>)> = vec![
        ("block map, nested map and list",
            "analyzer:\n  exclude:\n    - gen/**\n  errors:\n    todo: warning\n    a:\nlinter: ~\n",
            Ok(vec![e(&["analyzer"], false, 1, Nested), e(&["analyzer", "exclude"], false, 2, Nested),
                e(&["analyzer", "exclude"], true, 3, p("gen/**")), e(&["analyzer", "errors"], false, 4, Nested),
                e(&["analyzer", "errors", "todo"], false, 5, p("warning")), e(&["analyzer", "errors", "a"], false, 6, Null),
                e(&["linter"], false, 7, Null)])),
        ("tagged scalars", "a: !!str true\nb: !!bool true\n", Ok(vec![e(&["a"], false, 1, q("true")), e(&["b"], false, 2, q("true"))])),
        ("quoted and plain true", "a: true\nb: 'true'\n", Ok(vec![e(&["a"], false, 1, p("true")), e(&["b"], false, 2, q("true"))])),
        ("escaped key and value", "\"\\x69nclude\": \"\\x69gnore\"\n", Ok(vec![e(&["include"], false, 1, q("ignore"))])),
        ("two-line quoted in a flow map", "a:\n  b: {x: \"a\n    # b\", y: ignore}\n",
            Ok(vec![e(&["a"], false, 1, Nested), e(&["a", "b"], false, 2, Nested), e(&["a", "b", "x"], false, 2, q("a # b")),
                e(&["a", "b", "y"], false, 3, p("ignore"))])),
        ("flow map and flow sequence", "{a: [x, 'y'], b: {c: 1}}\n",
            Ok(vec![e(&["a"], false, 1, Nested), e(&["a"], true, 1, p("x")), e(&["a"], true, 1, q("y")),
                e(&["b"], false, 1, Nested), e(&["b", "c"], false, 1, p("1"))])),
        ("block scalar", "a: |\n  x\n  y\n", Ok(vec![e(&["a"], false, 1, q("x\ny\n"))])),
        ("alias of a scalar", "a: &v ignore\nb: *v\n", Ok(vec![e(&["a"], false, 1, p("ignore")), e(&["b"], false, 2, p("ignore"))])),
        ("alias of a mapping", "base: &m\n  x: 1\n  y: [z]\nuse:\n  - *m\n",
            Ok(vec![e(&["base"], false, 1, Nested), e(&["base", "x"], false, 2, p("1")), e(&["base", "y"], false, 3, Nested),
                e(&["base", "y"], true, 3, p("z")), e(&["use"], false, 4, Nested), e(&["use"], true, 5, Nested),
                e(&["use", "x"], false, 5, p("1")), e(&["use", "y"], false, 5, Nested), e(&["use", "y"], true, 5, p("z"))])),
        ("alias of an anchored key", "&k a: 1\nb: *k\n", Ok(vec![e(&["a"], false, 1, p("1")), e(&["b"], false, 2, p("a"))])),
        ("nulls", "a: ~\nb: null\nc:\nd: NULL\n",
            Ok(vec![e(&["a"], false, 1, Null), e(&["b"], false, 2, Null), e(&["c"], false, 3, Null), e(&["d"], false, 4, Null)])),
        ("byte-order mark", "\u{feff}include: a.yaml\n", Ok(vec![e(&["include"], false, 1, p("a.yaml"))])),
        ("empty text", "", Ok(vec![])),
        ("only comments", "# a\n\n# b\n", Ok(vec![])),
        ("quote left open", "a: \"x\n", Err((1, "quoted scalar"))),
        ("bad indentation", "a:\n    b: 1\n  c: 2\n", Err((3, "did not find expected key"))),
        ("two documents", "a: 1\n---\nb: 2\n", Err((2, "a second YAML document"))),
        ("explicit scalar key", "? a\n: b\n", Ok(vec![e(&["a"], false, 2, p("b"))])),
        ("complex key", "? [a, b]\n: c\n", Err((1, "a key that is not a scalar"))),
        ("flow sequence as a key", "[a]: b\n", Err((1, "a key that is not a scalar"))),
        ("duplicate key", "a: 1\na: 2\n", Err((2, "duplicate mapping key `a`"))),
        ("duplicate key, decoded", "a:\n  x: 1\n  \"\\x78\": 2\n", Err((3, "duplicate mapping key `x`"))),
        ("null key twice", "~: 1\nnull: 2\n", Err((2, "duplicate mapping key `null`"))),
        ("true key twice", "true: 1\nTrue: 2\n", Err((2, "duplicate mapping key `True`"))),
        ("tagged and plain true", "!!bool true: 1\ntrue: 2\n", Err((2, "duplicate mapping key `true`"))),
        ("int in decimal and hex", "1: a\n0x1: b\n", Err((2, "duplicate mapping key `0x1`"))),
        ("int and float", "1: a\n1.0: b\n", Err((2, "duplicate mapping key `1.0`"))),
        ("quoted and plain word", "\"a\": 1\na: 2\n", Err((2, "duplicate mapping key `a`"))),
        ("quoted and plain int, two keys", "\"1\": a\n1: b\n", Ok(vec![e(&["1"], false, 1, p("a")), e(&["1"], false, 2, p("b"))])),
        ("non-specific tag and quoted", "! 1: a\n\"1\": b\n", Err((2, "duplicate mapping key `1`"))),
        ("non-specific tag and plain int, two keys", "! 1: a\n1: b\n", Ok(vec![e(&["1"], false, 1, p("a")), e(&["1"], false, 2, p("b"))])),
        ("local tag and plain word", "!foo a: 1\na: 2\n", Err((2, "duplicate mapping key `a`"))),
        (".nan twice", ".nan: 1\n.nan: 2\n", Err((2, "duplicate mapping key `.nan`"))),
        ("int tag on a word", "a: !!int abc\n", Err((1, "`!!int`"))),
        ("bool tag on a word, as a key", "!!bool maybe: 1\n", Err((1, "`!!bool`"))),
        ("seq tag on a scalar", "a: !!seq abc\n", Err((1, "`!!seq`"))),
        ("binary tag on a key", "!!binary abc: 1\n", Err((1, "`!!binary`"))),
        ("int tag on a mapping", "a: !!int {x: 1}\n", Err((1, "`!!int`"))),
        ("map tag on a mapping", "a: !!map {x: 1}\n", Ok(vec![e(&["a"], false, 1, Nested), e(&["a", "x"], false, 1, p("1"))])),
        ("local tag on a mapping", "a: !foo {x: 1}\n", Ok(vec![e(&["a"], false, 1, Nested), e(&["a", "x"], false, 1, p("1"))])),
        ("merge key", "a: &a {x: 1}\nb:\n  <<: *a\n", Err((3, "a merge key"))),
        ("alias inside its anchor's node", "a: &x [*x]\n", Err((1, "an alias inside"))),
        ("nested aliases past the bound", bomb.as_str(), Err((5, "more than 100000 YAML entries"))),
        ("64 levels", sixty_four.as_str(), Ok(vec![e(&[], true, 1, Nested); 63])),
        ("65 levels", sixty_five.as_str(), Err((1, "nested deeper than 64 levels"))),
    ];
    let mut failed = Vec::new();
    for (name, text, want) in rows {
        let got = yaml_entries(text);
        let ok = match (&got, &want) {
            (Ok(entries), Ok(want)) => entries == want,
            (Err(why), Err((line, what))) => why.starts_with(&format!("line {line}: ")) && why.contains(what),
            _ => false,
        };
        if !ok {
            eprintln!("{name}: got {got:?}");
            failed.push(name);
        }
    }
    assert!(failed.is_empty(), "failed rows: {failed:?}");
}

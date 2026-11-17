//! ZUT for rule_outcome.rs: the exact JSON text of each variant (external tagging, snake_case tags, `code` null when
//! None, never left out), and a toml round trip back to the same value (the JSON text is not read back). The crate
//! may depend only on serde, toml and globset, so serde_json is not available: a compact test-only JSON writer below
//! pins the JSON text and covers the serde calls these values make; any other call is an error.

use super::{Engine, Finding, Kind, RelPath, RuleOutcome, Severity};
use serde::ser::{self, Impossible, Serialize, Serializer};
use std::fmt;

#[derive(Debug)]
struct Unsupported(String);

impl fmt::Display for Unsupported {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for Unsupported {}

impl ser::Error for Unsupported {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Unsupported(msg.to_string())
    }
}

struct Json(String);

impl Json {
    fn string(&mut self, text: &str) {
        self.0.push('"');
        for c in text.chars() {
            if c == '"' || c == '\\' {
                self.0.push('\\');
            }
            self.0.push(c);
        }
        self.0.push('"');
    }
}

fn to_json<T: Serialize>(value: &T) -> String {
    let mut json = Json(String::new());
    value.serialize(&mut json).unwrap();
    json.0
}

macro_rules! unsupported {
    ($($name:ident($($arg:ty),*) -> $ret:ty;)*) => {
        $(fn $name(self, $(_: $arg),*) -> Result<$ret, Unsupported> {
            Err(Unsupported(stringify!($name).to_string()))
        })*
    };
}

impl<'a> Serializer for &'a mut Json {
    type Ok = ();
    type Error = Unsupported;
    type SerializeSeq = Compound<'a>;
    type SerializeTuple = Impossible<(), Unsupported>;
    type SerializeTupleStruct = Impossible<(), Unsupported>;
    type SerializeTupleVariant = Impossible<(), Unsupported>;
    type SerializeMap = Impossible<(), Unsupported>;
    type SerializeStruct = Compound<'a>;
    type SerializeStructVariant = Compound<'a>;

    unsupported! {
        serialize_bool(bool) -> ();
        serialize_i8(i8) -> ();
        serialize_i16(i16) -> ();
        serialize_i64(i64) -> ();
        serialize_u8(u8) -> ();
        serialize_u16(u16) -> ();
        serialize_u64(u64) -> ();
        serialize_f32(f32) -> ();
        serialize_f64(f64) -> ();
        serialize_char(char) -> ();
        serialize_bytes(&[u8]) -> ();
        serialize_unit() -> ();
        serialize_unit_struct(&'static str) -> ();
        serialize_tuple(usize) -> Self::SerializeTuple;
        serialize_tuple_struct(&'static str, usize) -> Self::SerializeTupleStruct;
        serialize_tuple_variant(&'static str, u32, &'static str, usize) -> Self::SerializeTupleVariant;
        serialize_map(Option<usize>) -> Self::SerializeMap;
    }

    fn serialize_i32(self, v: i32) -> Result<(), Unsupported> {
        self.0.push_str(&v.to_string());
        Ok(())
    }

    fn serialize_u32(self, v: u32) -> Result<(), Unsupported> {
        self.0.push_str(&v.to_string());
        Ok(())
    }

    fn serialize_str(self, v: &str) -> Result<(), Unsupported> {
        self.string(v);
        Ok(())
    }

    fn serialize_none(self) -> Result<(), Unsupported> {
        self.0.push_str("null");
        Ok(())
    }

    fn serialize_some<T: ?Sized + Serialize>(self, value: &T) -> Result<(), Unsupported> {
        value.serialize(self)
    }

    fn serialize_unit_variant(self, _: &'static str, _: u32, variant: &'static str) -> Result<(), Unsupported> {
        self.string(variant);
        Ok(())
    }

    fn serialize_newtype_struct<T: ?Sized + Serialize>(self, _: &'static str, value: &T) -> Result<(), Unsupported> {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized + Serialize>(
        self,
        _: &'static str,
        _: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<(), Unsupported> {
        self.0.push('{');
        self.string(variant);
        self.0.push(':');
        value.serialize(&mut *self)?;
        self.0.push('}');
        Ok(())
    }

    fn serialize_seq(self, _: Option<usize>) -> Result<Compound<'a>, Unsupported> {
        self.0.push('[');
        Ok(Compound { json: self, first: true, close: "]" })
    }

    fn serialize_struct(self, _: &'static str, _: usize) -> Result<Compound<'a>, Unsupported> {
        self.0.push('{');
        Ok(Compound { json: self, first: true, close: "}" })
    }

    fn serialize_struct_variant(
        self,
        _: &'static str,
        _: u32,
        variant: &'static str,
        _: usize,
    ) -> Result<Compound<'a>, Unsupported> {
        self.0.push('{');
        self.string(variant);
        self.0.push_str(":{");
        Ok(Compound { json: self, first: true, close: "}}" })
    }
}

struct Compound<'a> {
    json: &'a mut Json,
    first: bool,
    close: &'static str,
}

impl Compound<'_> {
    fn element<T: ?Sized + Serialize>(&mut self, key: Option<&str>, value: &T) -> Result<(), Unsupported> {
        if !self.first {
            self.json.0.push(',');
        }
        self.first = false;
        if let Some(key) = key {
            self.json.string(key);
            self.json.0.push(':');
        }
        value.serialize(&mut *self.json)
    }

    fn close(self) -> Result<(), Unsupported> {
        self.json.0.push_str(self.close);
        Ok(())
    }
}

impl ser::SerializeSeq for Compound<'_> {
    type Ok = ();
    type Error = Unsupported;
    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Unsupported> {
        self.element(None, value)
    }
    fn end(self) -> Result<(), Unsupported> {
        self.close()
    }
}

impl ser::SerializeStruct for Compound<'_> {
    type Ok = ();
    type Error = Unsupported;
    fn serialize_field<T: ?Sized + Serialize>(&mut self, key: &'static str, value: &T) -> Result<(), Unsupported> {
        self.element(Some(key), value)
    }
    fn end(self) -> Result<(), Unsupported> {
        self.close()
    }
}

impl ser::SerializeStructVariant for Compound<'_> {
    type Ok = ();
    type Error = Unsupported;
    fn serialize_field<T: ?Sized + Serialize>(&mut self, key: &'static str, value: &T) -> Result<(), Unsupported> {
        self.element(Some(key), value)
    }
    fn end(self) -> Result<(), Unsupported> {
        self.close()
    }
}

fn one_finding() -> RuleOutcome {
    RuleOutcome::Findings(vec![Finding {
        rule: "no-bool".to_string(),
        engine: Engine::AstGrep,
        kind: Kind::Quality,
        severity: Severity::Warning,
        owner: None,
        path: RelPath::new("src/a.rs"),
        line: 7,
        col: None,
        what: "stored Bool".to_string(),
        fix: "Store a bitmap field.".to_string(),
    }])
}

fn not_available() -> RuleOutcome {
    RuleOutcome::NotAvailable {
        rule: "no-bool".to_string(),
        engine: Engine::AstGrep,
        why: "ast-grep is not installed".to_string(),
    }
}

fn failed(code: Option<i32>) -> RuleOutcome {
    RuleOutcome::Failed { rule: "lint".to_string(), engine: Engine::Clippy, code, stderr: "bad \"args\"".to_string() }
}

#[test]
fn findings_serializes_as_a_findings_tag_over_the_list() {
    assert_eq!(
        to_json(&one_finding()),
        r#"{"findings":[{"rule":"no-bool","engine":"ast-grep","kind":"quality","severity":"warning","path":"src/a.rs","line":7,"what":"stored Bool","fix":"Store a bitmap field."}]}"#
    );
}

#[test]
fn no_findings_serializes_as_an_empty_list() {
    assert_eq!(to_json(&RuleOutcome::Findings(Vec::new())), r#"{"findings":[]}"#);
}

#[test]
fn not_available_serializes_with_its_snake_case_tag() {
    assert_eq!(
        to_json(&not_available()),
        r#"{"not_available":{"rule":"no-bool","engine":"ast-grep","why":"ast-grep is not installed"}}"#
    );
}

#[test]
fn failed_with_an_exit_code_serializes_the_number() {
    assert_eq!(
        to_json(&failed(Some(2))),
        r#"{"failed":{"rule":"lint","engine":"clippy","code":2,"stderr":"bad \"args\""}}"#
    );
}

#[test]
fn failed_without_an_exit_code_serializes_code_as_null() {
    assert_eq!(
        to_json(&failed(None)),
        r#"{"failed":{"rule":"lint","engine":"clippy","code":null,"stderr":"bad \"args\""}}"#
    );
}

#[test]
fn every_variant_reads_back_equal_through_toml() {
    for outcome in [one_finding(), RuleOutcome::Findings(Vec::new()), not_available(), failed(Some(2)), failed(None)] {
        let text = toml::to_string(&outcome).unwrap();
        let read: RuleOutcome = toml::from_str(&text).unwrap_or_else(|e| panic!("{e}\n{text}"));
        assert_eq!(read, outcome, "{text}");
    }
}

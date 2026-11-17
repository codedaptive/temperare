//! Test-only compact JSON writer for the report's types: serde_json is not a dependency (S-16), so this pins the
//! JSON text. It covers structs, sequences, strings (escaping only `"`, `\` and newline), u32, usize (as u64), i32,
//! None as null, Some, newtype structs and unit variants; any other serde call is an error.

use serde::ser::{self, Impossible, Serialize, Serializer};
use std::fmt;

#[derive(Debug)]
pub(crate) struct Unsupported(String);

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

pub(crate) struct Json(String);

impl Json {
    fn string(&mut self, text: &str) {
        self.0.push('"');
        for c in text.chars() {
            match c {
                '"' | '\\' => {
                    self.0.push('\\');
                    self.0.push(c);
                }
                '\n' => self.0.push_str("\\n"),
                _ => self.0.push(c),
            }
        }
        self.0.push('"');
    }

    fn number(&mut self, text: String) -> Result<(), Unsupported> {
        self.0.push_str(&text);
        Ok(())
    }
}

/// `value` as compact JSON text.
pub(crate) fn to_json<T: Serialize>(value: &T) -> String {
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
    type SerializeStructVariant = Impossible<(), Unsupported>;

    unsupported! {
        serialize_bool(bool) -> ();
        serialize_i8(i8) -> ();
        serialize_i16(i16) -> ();
        serialize_i64(i64) -> ();
        serialize_u8(u8) -> ();
        serialize_u16(u16) -> ();
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
        serialize_struct_variant(&'static str, u32, &'static str, usize) -> Self::SerializeStructVariant;
    }

    fn serialize_i32(self, v: i32) -> Result<(), Unsupported> {
        self.number(v.to_string())
    }

    fn serialize_u32(self, v: u32) -> Result<(), Unsupported> {
        self.number(v.to_string())
    }

    fn serialize_u64(self, v: u64) -> Result<(), Unsupported> {
        self.number(v.to_string())
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
        _: &T,
    ) -> Result<(), Unsupported> {
        Err(Unsupported(format!("serialize_newtype_variant {variant}")))
    }

    fn serialize_seq(self, _: Option<usize>) -> Result<Compound<'a>, Unsupported> {
        self.0.push('[');
        Ok(Compound { json: self, first: true, close: ']' })
    }

    fn serialize_struct(self, _: &'static str, _: usize) -> Result<Compound<'a>, Unsupported> {
        self.0.push('{');
        Ok(Compound { json: self, first: true, close: '}' })
    }
}

pub(crate) struct Compound<'a> {
    json: &'a mut Json,
    first: bool,
    close: char,
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
        self.json.0.push(self.close);
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
    fn skip_field(&mut self, _: &'static str) -> Result<(), Unsupported> {
        Ok(())
    }
    fn end(self) -> Result<(), Unsupported> {
        self.close()
    }
}

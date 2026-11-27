//! `json_settings`: a `.markdownlint.json` read into its top-level keys, in order, each with whether it turns rules on.

use serde::de::{self, DeserializeSeed, Deserializer, IgnoredAny, MapAccess, SeqAccess, Visitor};
use std::collections::HashSet;
use std::fmt;

/// The top-level keys of the JSON object `text`, in file order, each with what markdownlint 0.42.2's
/// `getEffectiveConfig` makes of its value, read as JavaScript's truthiness (false, null, a number equal to zero and
/// `""` are falsy; anything else, an array or object included, is truthy):
/// - a key equal to `default`, ignoring case: whether its value is truthy (`!!value`), an object included;
/// - any other key: false for a falsy value; for an object, true unless it has an `enabled` whose value is falsy; true
///   for any other truthy value.
///
/// serde_json here keeps no key order of its own, so the object is read key by key. A key that repeats at the top
/// level, or an `enabled` that repeats in the object of a key other than `default`, is an `Err`: JavaScript keeps the
/// first one's place and the last one's value, which cctao does not model. Under `default` (any case) only the value's
/// truthiness counts, so its object's keys are skipped unread and a repeated `enabled` there is no `Err`. Text that is not one JSON object (JSONC comments and trailing commas
/// included, which markdownlint-cli2 would accept) is an `Err` with serde_json's message, which names the line.
pub(crate) fn json_settings(text: &str) -> Result<Vec<(String, bool)>, String> {
    let mut reader = serde_json::Deserializer::from_str(text);
    let settings = reader.deserialize_map(TopLevel).map_err(|err| err.to_string())?;
    reader.end().map_err(|err| err.to_string())?;
    Ok(settings)
}

/// Reads the top-level object.
struct TopLevel;

impl<'de> Visitor<'de> for TopLevel {
    type Value = Vec<(String, bool)>;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("a JSON object")
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut seen = HashSet::new();
        let mut settings = Vec::new();
        while let Some(key) = map.next_key::<String>()? {
            if !seen.insert(key.clone()) {
                return Err(de::Error::custom(format!("duplicate key `{key}`")));
            }
            let on = map.next_value_seed(On { truthy_only: key.eq_ignore_ascii_case("default") })?;
            settings.push((key, on));
        }
        Ok(settings)
    }
}

/// Reads one value as on or off. `truthy_only`: the value's truthiness alone (`default`, and `enabled` itself);
/// otherwise an object is on unless its `enabled` is falsy.
struct On {
    truthy_only: bool,
}

impl<'de> DeserializeSeed<'de> for On {
    type Value = bool;

    fn deserialize<D: Deserializer<'de>>(self, reader: D) -> Result<bool, D::Error> {
        reader.deserialize_any(self)
    }
}

impl<'de> Visitor<'de> for On {
    type Value = bool;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("a JSON value")
    }

    fn visit_bool<E>(self, value: bool) -> Result<bool, E> {
        Ok(value)
    }

    fn visit_i64<E>(self, value: i64) -> Result<bool, E> {
        Ok(value != 0)
    }

    fn visit_u64<E>(self, value: u64) -> Result<bool, E> {
        Ok(value != 0)
    }

    fn visit_f64<E>(self, value: f64) -> Result<bool, E> {
        Ok(value != 0.0)
    }

    fn visit_str<E>(self, value: &str) -> Result<bool, E> {
        Ok(!value.is_empty())
    }

    fn visit_unit<E>(self) -> Result<bool, E> {
        Ok(false)
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<bool, A::Error> {
        while seq.next_element::<IgnoredAny>()?.is_some() {}
        Ok(true)
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<bool, A::Error> {
        let mut enabled = None;
        while let Some(key) = map.next_key::<String>()? {
            if self.truthy_only || key != "enabled" {
                map.next_value::<IgnoredAny>()?;
            } else if enabled.is_some() {
                return Err(de::Error::custom("duplicate key `enabled`"));
            } else {
                enabled = Some(map.next_value_seed(On { truthy_only: true })?);
            }
        }
        Ok(enabled.unwrap_or(true))
    }
}

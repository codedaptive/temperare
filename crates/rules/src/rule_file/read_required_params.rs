//! `read_required_params`: the `required_params` of a baseline set's `[[rule]]`.

use super::check_keys::check_keys;
use super::param_type::ParamType;
use super::problem_at::problem_at;
use super::required_string::required_string;
use super::wrong_type::wrong_type;
use crate::Problem;
use toml::Spanned;
use toml::de::{DeTable, DeValue};

const KEY: &str = "required_params";
const EXPECTED: &str = "an array of param names or { name, type } tables";

/// The rule's required params in order, each with its declared type: a string item is the name alone, untyped (None);
/// a `{ name, type }` table gives the type, "string" or "strings". Absent is empty. A value that is not an array is a
/// Problem; an item that is neither a string nor such a table, or a table with a Problem, is a Problem and left out.
pub(crate) fn read_required_params(
    text: &str,
    table: &DeTable,
    out: &mut Vec<Problem>,
) -> Vec<(String, Option<ParamType>)> {
    let Some(value) = table.get(KEY) else {
        return Vec::new();
    };
    let Some(items) = value.get_ref().as_array() else {
        out.push(wrong_type(text, KEY, value, EXPECTED));
        return Vec::new();
    };
    items.iter().filter_map(|item| read_item(text, item, out)).collect()
}

/// One item of the array: a name, or a `{ name, type }` table; None, with its Problems in `out`, otherwise.
fn read_item(text: &str, item: &Spanned<DeValue>, out: &mut Vec<Problem>) -> Option<(String, Option<ParamType>)> {
    if let Some(name) = item.get_ref().as_str() {
        return Some((name.to_string(), None));
    }
    let Some(entry) = item.get_ref().as_table() else {
        out.push(wrong_type(text, KEY, item, EXPECTED));
        return None;
    };
    let before = out.len();
    check_keys(text, entry, &["name", "type"], KEY, out);
    let name = required_string(text, entry, "name", KEY, item.span().start, out);
    let ty = required_string(text, entry, "type", KEY, item.span().start, out);
    let ((name, _), (ty, ty_at)) = (name?, ty?);
    let ty = match ty.as_str() {
        "string" => ParamType::String,
        "strings" => ParamType::Strings,
        _ => {
            out.push(problem_at(
                text,
                ty_at,
                format!("unknown param type \"{ty}\" for required param \"{name}\""),
                "Write type = \"string\" or type = \"strings\".",
            ));
            return None;
        }
    };
    (out.len() == before).then_some((name, Some(ty)))
}

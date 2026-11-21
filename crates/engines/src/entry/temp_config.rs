//! `temp_config`: the temporary config file an entry takes a rule's params through.

use super::{param_why, Entry, Params, TempFile};
use crate::ParamError;
use serde::Serialize;
use serde_json::Value;
use std::ffi::OsString;

/// For `Params::Config(c)`: `c.text` of the params table, written with `TempFile::write(entry.name, c.ext, ..)`,
/// with the argument `{c.option}{path}`. The file is removed when the `TempFile` is dropped. Every other `Params`
/// kind writes nothing. The error is the stderr of the rule's `Failed`: `param_why`'s text for params that are not
/// a table (`Bad` named `params`) or that `c.text` cannot use, and `cannot write the <name> config: <error>` when
/// the file cannot be written.
pub(crate) fn temp_config<P: Serialize>(entry: &Entry, params: &P) -> Result<Option<(TempFile, OsString)>, String> {
    let Params::Config(config) = entry.params else { return Ok(None) };
    let Ok(Value::Object(table)) = serde_json::to_value(params) else {
        let err = ParamError::Bad { name: "params".to_string(), why: "not a table of named params".to_string() };
        return Err(param_why(entry.name, err));
    };
    let text = (config.text)(&table).map_err(|err| param_why(entry.name, err))?;
    let temp = TempFile::write(entry.name, config.ext, &text)
        .map_err(|err| format!("cannot write the {} config: {err}", entry.name))?;
    let mut arg = OsString::from(config.option);
    arg.push(&temp.path);
    Ok(Some((temp, arg)))
}

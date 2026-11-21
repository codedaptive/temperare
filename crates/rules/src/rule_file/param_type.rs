//! `ParamType`: the type a baseline set declares for one of a rule's required params.

/// The TOML type an `[[override]]`'s params must give a typed required param: `String` a TOML string, `Strings` an
/// array of strings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ParamType {
    String,
    Strings,
}

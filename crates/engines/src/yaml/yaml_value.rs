//! `YamlValue`: what one node of a YAML text holds, as `yaml_entries` gives it.

/// One node's value. `Null` is an untagged plain scalar that is empty, `~`, `null`, `Null` or `NULL` (the YAML 1.2 core
/// schema); `Plain` any other untagged plain scalar; `Quoted` a scalar in any other style (single, double, literal or
/// folded) or carrying any tag (`!!str true`, `!!bool true`), its text as the parser decoded it, escapes resolved;
/// `Nested` a mapping or a sequence, whose entries follow it. Plain and quoted stay apart because YAML reads a plain
/// `true` as a boolean and a quoted one as a string. A tag, not the core schema, decides what a tagged scalar means,
/// so it is never read as `Plain` or `Null`: a reader that acts only on plain values does not act on it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum YamlValue {
    Null,
    Plain(String),
    Quoted(String),
    Nested,
}

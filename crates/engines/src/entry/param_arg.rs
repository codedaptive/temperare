//! `ParamArg`: one param an engine takes as arguments.

/// A param named `name` whose list items each become one argument `{flag}{item}`. The flag carries its own `=`
/// (`"--allow="`), so an item beginning with `-` is read as the flag's value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParamArg {
    /// The param's name in the rule file.
    pub name: &'static str,
    /// What each item is joined to.
    pub flag: &'static str,
}

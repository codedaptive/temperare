//! `Threshold`: one numeric limit a rule may set for one check of an engine.

/// A param `name` taking a whole number from 1 to `max`, read only for the check `check`. `setting(n)` is the text
/// the engine is given for it: an argument's value, or a temporary file's whole text, as the entry's `Reach` says.
#[derive(Debug, Clone, Copy)]
pub struct Threshold {
    /// The param's name in the rule file ("max_complexity").
    pub name: &'static str,
    /// The check the param is read for, with `_` where the rule's check may write `-`.
    pub check: &'static str,
    /// The largest number the param takes.
    pub max: u32,
    /// The text the engine is given for the number.
    pub setting: fn(u32) -> String,
}

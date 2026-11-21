//! `CountMatch`: one ast-grep match as count mode reads it.

/// One match line of `ast-grep scan --json=stream`, with the fields counting needs. Lines and columns are 0-based
/// and byte offsets are into the file, all as ast-grep prints them.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct CountMatch {
    /// The path as ast-grep printed it.
    pub(crate) file: String,
    /// The matched source text.
    pub(crate) text: String,
    pub(crate) start_byte: usize,
    pub(crate) end_byte: usize,
    pub(crate) start_line: u32,
    pub(crate) end_line: u32,
    pub(crate) end_column: u32,
}

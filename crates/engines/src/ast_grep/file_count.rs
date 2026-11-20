//! `FileCount`: one file's setup and test lines.

/// The non-blank lines of one file, split into those outside every test function (setup) and those inside one.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct FileCount {
    pub(crate) setup: u32,
    pub(crate) tests: u32,
}

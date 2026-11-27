//! `CaseFile`: one file of a baseline rule's failing or passing case, carried in cctao.

/// One file of a case, as cctao carries it with the baseline set.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CaseFile {
    /// Where the file is written, relative to the folder the case is run in.
    pub path: String,
    /// The file's text, as the set carries it.
    pub text: &'static str,
}

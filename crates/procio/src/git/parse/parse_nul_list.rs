//! `parse_nul_list`: the entries of git's NUL-separated output.

/// The entries of git's `-z` output, bytes kept exactly.
///
/// Each entry ends with NUL; a last entry without one is still an entry. Empty entries are dropped, so empty
/// input gives no entries.
pub(crate) fn parse_nul_list(bytes: &[u8]) -> Vec<Vec<u8>> {
    bytes.split(|byte| *byte == 0).filter(|entry| !entry.is_empty()).map(<[u8]>::to_vec).collect()
}

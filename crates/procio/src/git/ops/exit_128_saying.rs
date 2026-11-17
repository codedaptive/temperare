//! `exit_128_saying`: whether git failed with its fatal exit code and one of the given messages.

use crate::Output;

/// True when `out` is exit 128 and its stderr holds any of `messages` byte for byte. git runs with `LC_ALL=C`, so
/// its fatal messages are the English text the operations match on.
pub(crate) fn exit_128_saying(out: &Output, messages: &[&[u8]]) -> bool {
    out.code == Some(128)
        && messages.iter().any(|m| !m.is_empty() && out.stderr.windows(m.len()).any(|w| w == *m))
}

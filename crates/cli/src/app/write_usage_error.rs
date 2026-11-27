//! `write_usage_error`: a usage error as cctao prints it on stderr.

use crate::UsageError;
use std::io::Write;

/// Writes `cctao: <what>` and `next: <fix>`, each on its own line. A write error is ignored: a closed pipe is not
/// cctao's failure, and there is nowhere left to say it.
pub(crate) fn write_usage_error(err: &mut dyn Write, e: &UsageError) {
    let _ = writeln!(err, "cctao: {}\nnext: {}", e.what, e.fix);
}

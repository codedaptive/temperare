//! `write_usage_error`: a usage error as tm prints it on stderr.

use crate::UsageError;
use std::io::Write;

/// Writes `tm: <what>` and `next: <fix>`, each on its own line. A write error is ignored: a closed pipe is not
/// tm's failure, and there is nowhere left to say it.
pub(crate) fn write_usage_error(err: &mut dyn Write, e: &UsageError) {
    let _ = writeln!(err, "tm: {}\nnext: {}", e.what, e.fix);
}

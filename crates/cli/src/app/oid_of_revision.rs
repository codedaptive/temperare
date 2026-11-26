//! `oid_of_revision`: the revision git named, as a commit id.

use ccvcslib::{Revision, VcsError};
use report::Oid;

/// No revision is `Ok(None)`; a revision whose text is a commit id (40 or 64 hex digits) is that id. Any other text
/// is `VcsError::Failed` naming it, never `Ok(None)`: git did name something, so the base is not simply absent.
pub(crate) fn oid_of_revision(answer: Option<Revision>) -> Result<Option<Oid>, VcsError> {
    let Some(rev) = answer else {
        return Ok(None);
    };
    match Oid::parse(&rev.0) {
        Some(oid) => Ok(Some(oid)),
        None => Err(VcsError::Failed(format!("git named {}, not a commit id", rev.0).into())),
    }
}

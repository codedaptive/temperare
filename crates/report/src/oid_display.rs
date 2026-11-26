//! `Display` for `Oid`: the id's text, as git would print it.

use std::fmt;

use crate::Oid;

impl fmt::Display for Oid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

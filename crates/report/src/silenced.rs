//! `Silenced`: a finding an exception covers, with that exception.

use rules::{Exception, Finding, RelPath};
use serde::{Serialize, Serializer};

/// A finding on a new line that an exception in the rule file covers. The exception serializes as `rule`,
/// `path`, `what` (left out when the exception has none), `reason` and `approved`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Silenced {
    pub finding: Finding,
    #[serde(serialize_with = "exception_fields")]
    pub exception: Exception,
}

/// The exception's fields, borrowed: `rules::Exception` has no `Serialize`.
#[derive(Serialize)]
struct ExceptionFields<'a> {
    rule: &'a str,
    path: &'a RelPath,
    #[serde(skip_serializing_if = "Option::is_none")]
    what: Option<&'a str>,
    reason: &'a str,
    approved: &'a str,
}

fn exception_fields<S: Serializer>(exception: &Exception, serializer: S) -> Result<S::Ok, S::Error> {
    ExceptionFields {
        rule: &exception.rule,
        path: &exception.path,
        what: exception.what.as_deref(),
        reason: &exception.reason,
        approved: &exception.approved,
    }
    .serialize(serializer)
}

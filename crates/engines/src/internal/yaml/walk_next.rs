//! `Walk::next`: the parser's next event. Tested through yaml_entries (yaml_entries_tests.rs), so no test file of its
//! own.

use super::walk::Walk;
use saphyr_parser::{Event, Span};

impl<'a> Walk<'a> {
    /// The next event, or the parser's error as `line N: <its message>`; the parser gives none only after the stream's
    /// end.
    pub(super) fn next(&mut self) -> Result<(Event<'a>, Span), String> {
        match self.parser.next_event() {
            Some(Ok(event)) => Ok(event),
            Some(Err(err)) => Err(format!("line {}: {}", err.marker().line(), err.info())),
            None => Ok((Event::StreamEnd, Span::default())),
        }
    }
}

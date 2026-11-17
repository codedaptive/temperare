//! `wait_deadline`: poll until an answer comes or the time is up.

use std::io;
use std::thread;
use std::time::{Duration, Instant};

/// The longest sleep between polls. std's `Child` has no wait-with-timeout, so `run`, with a timeout, polls
/// `try_wait`; 10 ms keeps a finished child's answer prompt without spinning.
const POLL_STEP: Duration = Duration::from_millis(10);

/// Calls `poll` until it gives `Some` (`Ok(Some)`) or, after a poll, `timeout` has passed since the call
/// (`Ok(None)`). A poll error is returned at once. Always polls at least once, even with a zero timeout.
pub(crate) fn wait_deadline<T>(
    mut poll: impl FnMut() -> io::Result<Option<T>>,
    timeout: Duration,
) -> io::Result<Option<T>> {
    // A timeout too large to add to now has no deadline that could ever pass.
    let deadline = Instant::now().checked_add(timeout);
    loop {
        if let Some(answer) = poll()? {
            return Ok(Some(answer));
        }
        let remaining = match deadline {
            Some(deadline) => deadline.saturating_duration_since(Instant::now()),
            None => POLL_STEP,
        };
        if remaining.is_zero() {
            return Ok(None);
        }
        thread::sleep(remaining.min(POLL_STEP));
    }
}

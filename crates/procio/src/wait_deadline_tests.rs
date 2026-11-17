use std::io;
use std::time::Duration;

use crate::wait_deadline::wait_deadline;

#[test]
fn an_answer_on_the_third_poll_is_returned_after_three_polls() {
    let mut polls = 0;
    let answer = wait_deadline(
        || {
            polls += 1;
            Ok((polls == 3).then_some("done"))
        },
        Duration::from_secs(60),
    );
    assert_eq!(answer.unwrap(), Some("done"));
    assert_eq!(polls, 3);
}

#[test]
fn a_zero_timeout_polls_once_then_gives_none() {
    let mut polls = 0;
    let answer = wait_deadline(
        || {
            polls += 1;
            Ok(None::<u8>)
        },
        Duration::ZERO,
    );
    assert_eq!(answer.unwrap(), None);
    assert_eq!(polls, 1);
}

#[test]
fn a_zero_timeout_still_takes_an_answer_from_the_first_poll() {
    assert_eq!(wait_deadline(|| Ok(Some(7)), Duration::ZERO).unwrap(), Some(7));
}

#[test]
fn a_poll_error_is_returned_at_once() {
    let mut polls = 0;
    let answer = wait_deadline(
        || {
            polls += 1;
            Err::<Option<u8>, _>(io::Error::other("wait failed"))
        },
        Duration::from_secs(60),
    );
    assert_eq!(answer.unwrap_err().to_string(), "wait failed");
    assert_eq!(polls, 1);
}

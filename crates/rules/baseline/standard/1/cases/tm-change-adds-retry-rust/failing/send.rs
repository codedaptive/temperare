/// The server's answer when it cannot take the request now.
pub struct Busy;

/// Sends the request, sending it again while the server is busy.
pub fn send(mut send_once: impl FnMut() -> Result<String, Busy>) -> Result<String, Busy> {
    let mut attempts = 0;
    loop {
        match send_once() {
            Err(Busy) if attempts < 3 => attempts += 1,
            answer => return answer,
        }
    }
}

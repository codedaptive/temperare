use std::sync::mpsc;
use std::thread;

/// Starts the worker and waits until it says it is up.
pub fn start_worker() -> Result<thread::JoinHandle<()>, mpsc::RecvError> {
    let (ready, started) = mpsc::channel();
    let handle = thread::spawn(move || {
        let _ = ready.send(());
    });
    started.recv()?;
    Ok(handle)
}

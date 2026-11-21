use std::sync::mpsc;
use std::thread;
use std::time::Duration;

/// Starts the worker and gives it time to come up.
pub fn start_worker() -> (thread::JoinHandle<()>, mpsc::Receiver<()>) {
    let (ready, started) = mpsc::channel();
    let handle = thread::spawn(move || {
        let _ = ready.send(());
    });
    thread::sleep(Duration::from_millis(200));
    (handle, started)
}

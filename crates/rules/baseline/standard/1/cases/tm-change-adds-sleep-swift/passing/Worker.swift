import Foundation

/// Starts the worker and waits until it says it is up.
func startWorker(_ worker: Worker) async {
    await withCheckedContinuation { (ready: CheckedContinuation<Void, Never>) in
        worker.start(onReady: { ready.resume() })
    }
}

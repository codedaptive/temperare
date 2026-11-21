import Foundation

/// Starts the worker and gives it time to come up.
func startWorker(_ worker: Worker) {
    worker.start()
    Thread.sleep(forTimeInterval: 0.2)
}

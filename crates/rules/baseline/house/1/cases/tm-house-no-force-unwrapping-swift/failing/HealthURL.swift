import Foundation

/// The address of the health check on `host`.
func healthURL(host: String) -> URL {
    URL(string: "https://\(host)/health")!
}

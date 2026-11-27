import Foundation

/// The address of the health check on `host`, or nil when `host` makes no valid URL.
func healthURL(host: String) -> URL? {
    URL(string: "https://\(host)/health")
}

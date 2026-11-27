import Foundation

/// The text of the settings file at `url`.
func settingsText(at url: URL) -> String {
    try! String(contentsOf: url, encoding: .utf8)
}

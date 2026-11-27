import Foundation

/// The text of the settings file at `url`.
func settingsText(at url: URL) throws -> String {
    try String(contentsOf: url, encoding: .utf8)
}

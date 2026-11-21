import Foundation

/// Reads the settings file; with no file, the defaults stand.
func readSettings(at url: URL, defaults: [String: String]) throws -> [String: String] {
    guard FileManager.default.fileExists(atPath: url.path) else {
        return defaults
    }
    let data = try Data(contentsOf: url)
    return try JSONDecoder().decode([String: String].self, from: data)
}

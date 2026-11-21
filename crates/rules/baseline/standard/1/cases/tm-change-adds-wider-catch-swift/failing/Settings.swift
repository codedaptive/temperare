import Foundation

/// Reads the settings file; with no file, the defaults stand.
func readSettings(at url: URL, defaults: [String: String]) -> [String: String] {
    do {
        let data = try Data(contentsOf: url)
        return try JSONDecoder().decode([String: String].self, from: data)
    } catch {
        return defaults
    }
}

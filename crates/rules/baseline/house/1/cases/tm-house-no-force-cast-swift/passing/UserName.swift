/// The "name" field of a decoded JSON object, or nil when it is missing or not text.
func userName(in object: [String: Any]) -> String? {
    object["name"] as? String
}

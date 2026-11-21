/// The "name" field of a decoded JSON object.
func userName(in object: [String: Any]) -> String {
    object["name"] as! String
}

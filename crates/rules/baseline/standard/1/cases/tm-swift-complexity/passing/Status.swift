/// A short description of HTTP status `code`.
func statusText(_ code: Int) -> String {
    switch code {
    case 200:
        return "ok"
    case 201:
        return "created"
    case 204:
        return "no content"
    case 301:
        return "moved permanently"
    case 400:
        return "bad request"
    case 401:
        return "unauthorized"
    case 403:
        return "forbidden"
    case 404:
        return "not found"
    case 500:
        return "server error"
    default:
        return "unknown"
    }
}

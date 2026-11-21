"""Text for HTTP status codes."""


def status_text(code):
    """A short description of HTTP status `code`."""
    if code == 200:
        return "ok"
    if code == 201:
        return "created"
    if code == 204:
        return "no content"
    if code == 301:
        return "moved permanently"
    if code == 400:
        return "bad request"
    if code == 401:
        return "unauthorized"
    if code == 403:
        return "forbidden"
    if code == 404:
        return "not found"
    if code == 500:
        return "server error"
    return "unknown"

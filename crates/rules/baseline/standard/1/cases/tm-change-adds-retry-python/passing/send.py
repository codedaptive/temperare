class Busy(Exception):
    """The server's answer when it cannot take the request now."""


def send(send_once):
    """Send the request once; a busy server is the caller's answer."""
    return send_once()


def read_pages(read_page):
    """Read every page, counting pages until the server has no more."""
    pages = []
    page = 0
    while (text := read_page(page)) is not None:
        pages.append(text)
        page += 1
    return pages

class Busy(Exception):
    """The server's answer when it cannot take the request now."""


def send(send_once):
    """Send the request, sending it again while the server is busy."""
    for attempt in range(3):
        try:
            return send_once()
        except Busy:
            if attempt == 2:
                raise

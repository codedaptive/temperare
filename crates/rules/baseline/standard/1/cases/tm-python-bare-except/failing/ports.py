"""Port numbers from text."""


def parse_port(text, default=8080):
    """The port number in `text`, or `default` when it is not a number."""
    try:
        return int(text)
    except:
        return default

"""The port a server listens on."""


def parse_port(text) -> int:
    return int(text.strip())

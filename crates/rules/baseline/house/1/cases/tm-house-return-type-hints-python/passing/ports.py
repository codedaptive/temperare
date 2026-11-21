"""The port a server listens on."""


def parse_port(text: str) -> int:
    return int(text.strip())

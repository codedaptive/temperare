import pytest

from config import parse_config


@pytest.mark.parametrize(
    ("text", "port"),
    [("[server]\nport = 8080\n", 8080), ("[server]\nport = 1\n", 1)],
)
def test_reads_the_port(text, port):
    assert parse_config(text).port == port

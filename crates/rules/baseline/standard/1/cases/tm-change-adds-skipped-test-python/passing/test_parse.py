def parse_port(text):
    port = int(text)
    return port if port < 65536 else None


def test_reads_a_port():
    assert parse_port("8080") == 8080


def test_refuses_a_port_out_of_range():
    assert parse_port("70000") is None

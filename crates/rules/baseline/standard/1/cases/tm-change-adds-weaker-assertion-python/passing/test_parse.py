def parse_port(text):
    return int(text)


def test_reads_a_port():
    assert parse_port("8080") == 8080

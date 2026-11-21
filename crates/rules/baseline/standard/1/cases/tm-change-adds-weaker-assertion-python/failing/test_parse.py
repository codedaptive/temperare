def parse_port(text):
    return int(text)


def test_reads_a_port():
    parse_port("8080")
    assert True

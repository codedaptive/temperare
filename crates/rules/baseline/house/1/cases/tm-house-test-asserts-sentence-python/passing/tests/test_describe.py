from ports import describe_port


def test_describes_an_open_port():
    description = describe_port(8080)
    assert description.port == 8080
    assert description.is_open, "port 8080 should be open for connections"

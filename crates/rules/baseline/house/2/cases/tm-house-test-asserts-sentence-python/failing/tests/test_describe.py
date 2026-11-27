from ports import describe_port


def test_describes_an_open_port():
    assert describe_port(8080) == "port 8080 is open for connections"

import textwrap

from config import parse_config


def config_text(port, host):
    return textwrap.dedent(
        f"""\
        [server]
        port = {port}
        host = "{host}"
        """
    )


def test_reads_the_port():
    assert parse_config(config_text(8080, "localhost")).port == 8080

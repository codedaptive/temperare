from unittest import mock

from ledger.store import send_all


def test_sends_every_line():
    with mock.patch("requests.post") as post:
        send_all(["a", "b"], url="https://ledger.example/lines")
    assert post.call_count == 2

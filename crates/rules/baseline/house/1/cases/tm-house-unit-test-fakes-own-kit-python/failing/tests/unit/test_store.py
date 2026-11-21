from unittest import mock

from ledger.store import save_all


def test_saves_every_line():
    with mock.patch("ledger.store.write_line") as write_line:
        save_all(["a", "b"])
    assert write_line.call_count == 2

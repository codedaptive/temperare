def litres(millilitres: int) -> int:
    """The reading of a meter in whole litres."""
    # A part litre is not billed.
    # Rounds down, so a customer never pays for water not yet used.
    return millilitres // 1000


def test_rounds_down() -> None:
    assert litres(1999) == 1, "a part litre is dropped"

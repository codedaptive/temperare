"""Order totals."""


def order_total(prices, tax_rate):
    """The total of `prices` with tax at `tax_rate`."""
    subtotal = sum(prices)
    # Tax is charged on the whole subtotal, not per item.
    return subtotal * (1 + tax_rate)

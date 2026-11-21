"""Order totals."""


def order_total(prices, tax_rate):
    """The total of `prices` with tax at `tax_rate`."""
    subtotal = sum(prices)
    return subtotal * (1 + tax_rate)

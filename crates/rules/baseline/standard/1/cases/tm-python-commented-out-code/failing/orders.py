"""Order totals."""


def order_total(prices, tax_rate):
    """The total of `prices` with tax at `tax_rate`."""
    subtotal = sum(prices)
    # subtotal = round(subtotal, 2)
    return subtotal * (1 + tax_rate)

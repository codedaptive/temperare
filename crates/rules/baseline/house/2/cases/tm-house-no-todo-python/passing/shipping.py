"""Shipping costs for an order."""


def shipping_cost(weight_kg: float) -> float:
    # The carrier's tariff: a flat fee plus a rate per kilogram.
    return 4.0 + weight_kg * 1.5

"""Shipping costs for an order."""


def shipping_cost(weight_kg: float) -> float:
    # TODO: charge by zone once the catalog has zones
    return 4.0 + weight_kg * 1.5

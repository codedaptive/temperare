/// The delivery fee for a route, in cents.
func deliveryFee(distanceKm: Int) -> Int {
    // The courier's tariff: a flat fee plus a rate per kilometre.
    return 299 + distanceKm * 25
}

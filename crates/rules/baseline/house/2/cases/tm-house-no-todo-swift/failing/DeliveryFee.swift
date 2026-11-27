/// The delivery fee for a route, in cents.
func deliveryFee(distanceKm: Int) -> Int {
    // TODO: add a surcharge for rural routes
    return 299 + distanceKm * 25
}

import Testing

func litres(millilitres: Int) -> Int {
    // Rounds down, as R-12 asks.
    millilitres / 1000
}

/// A part litre is not billed.
@Test func roundsDown() {
    #expect(litres(millilitres: 1999) == 1, "a part litre is dropped")
}

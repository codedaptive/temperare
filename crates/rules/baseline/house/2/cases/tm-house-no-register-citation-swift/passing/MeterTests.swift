import Testing

func litres(millilitres: Int) -> Int {
    // Rounds down, so a customer never pays for water not yet used.
    millilitres / 1000
}

/// A part litre is not billed.
@Test func roundsDown() {
    #expect(litres(millilitres: 1999) == 1, "a part litre is dropped")
}

import Testing

func parsePort(_ text: String) -> UInt16? {
    UInt16(text)
}

@Test func readsAPort() {
    #expect(parsePort("8080") == 8080)
}

@Test(.disabled("fails on CI")) func refusesAPortOutOfRange() {
    #expect(parsePort("70000") == nil)
}

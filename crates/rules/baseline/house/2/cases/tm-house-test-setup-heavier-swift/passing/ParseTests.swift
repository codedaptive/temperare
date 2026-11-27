import Testing
@testable import Config

@Test(arguments: [("[server]\nport = 8080\n", 8080), ("[server]\nport = 1\n", 1)])
func readsThePort(text: String, port: Int) throws {
    #expect(try parseConfig(text).port == port)
}

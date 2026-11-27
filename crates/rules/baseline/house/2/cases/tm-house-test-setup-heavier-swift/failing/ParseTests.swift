import Testing
@testable import Config

func configText(port: Int, host: String) -> String {
    """
    [server]
    port = \(port)
    host = "\(host)"
    """
}

@Test func readsThePort() throws {
    #expect(try parseConfig(configText(port: 8080, host: "localhost")).port == 8080)
}

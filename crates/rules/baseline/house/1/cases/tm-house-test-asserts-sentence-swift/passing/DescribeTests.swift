import Testing
import XCTest

struct Description {
    let port: Int
    let isOpen: Bool
}

func describePort(_ port: Int) -> Description {
    Description(port: port, isOpen: true)
}

@Test func describesAnOpenPort() {
    let description = describePort(8080)
    #expect(description.port == 8080)
    #expect(description.isOpen, "port 8080 should be open for connections")
}

struct NotAPort: Error {}

func parsePort(_ text: String) throws -> Int {
    guard let port = Int(text) else { throw NotAPort() }
    return port
}

final class ParsePortTests: XCTestCase {
    func testParsesANumber() {
        XCTAssertNoThrow(try parsePort("8080"), "a port written as digits should parse")
    }

    func testRefusesAWord() {
        XCTAssertThrowsError(try parsePort("http"), "a port written as a word should throw")
    }
}

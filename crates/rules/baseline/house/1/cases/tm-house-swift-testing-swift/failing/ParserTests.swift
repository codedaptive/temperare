import XCTest
@testable import Parser

final class ParserTests: XCTestCase {
    func testParsesOneLine() {
        XCTAssertEqual(Parser.lines(in: "a"), ["a"])
    }
}

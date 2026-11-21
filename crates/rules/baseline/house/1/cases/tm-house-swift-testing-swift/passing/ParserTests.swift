import Testing
@testable import Parser

struct ParserTests {
    @Test func parsesOneLine() {
        #expect(Parser.lines(in: "a") == ["a"])
    }
}

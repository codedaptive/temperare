import Testing

func describePort(_ port: Int) -> String {
    "port \(port) is open for connections"
}

@Test func describesAnOpenPort() {
    #expect(describePort(8080) == "port 8080 is open for connections")
}

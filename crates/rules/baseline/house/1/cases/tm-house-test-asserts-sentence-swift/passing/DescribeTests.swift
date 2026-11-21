import Testing

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

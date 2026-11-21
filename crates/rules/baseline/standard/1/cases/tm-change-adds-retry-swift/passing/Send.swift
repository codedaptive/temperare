/// The server's answer when it cannot take the request now.
struct Busy: Error {}

/// Sends the request once; a busy server is the caller's answer.
func send(_ sendOnce: () throws -> String) throws -> String {
    try sendOnce()
}

/// Reads every page, counting pages until the server has no more.
func readPages(_ readPage: (Int) -> String?) -> [String] {
    var pages: [String] = []
    var page = 0
    while let text = readPage(page) {
        pages.append(text)
        page += 1
    }
    return pages
}

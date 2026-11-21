/// The server's answer when it cannot take the request now.
struct Busy: Error {}

/// Sends the request, sending it again while the server is busy.
func send(_ sendOnce: () throws -> String) throws -> String {
    for attempt in 1...3 {
        do {
            return try sendOnce()
        } catch is Busy where attempt < 3 {
            continue
        }
    }
    throw Busy()
}

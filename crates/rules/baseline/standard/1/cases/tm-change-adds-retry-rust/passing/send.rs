/// The server's answer when it cannot take the request now.
pub struct Busy;

/// Sends the request once; a busy server is the caller's answer.
pub fn send(mut send_once: impl FnMut() -> Result<String, Busy>) -> Result<String, Busy> {
    send_once()
}

/// Reads every page, counting pages until the server has no more.
pub fn read_pages(mut read_page: impl FnMut(u32) -> Option<String>) -> Vec<String> {
    let mut pages = Vec::new();
    let mut page = 0;
    while let Some(text) = read_page(page) {
        pages.push(text);
        page += 1;
    }
    pages
}

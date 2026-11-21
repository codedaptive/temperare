package send

import "errors"

// ErrBusy is the server's answer when it cannot take the request now.
var ErrBusy = errors.New("busy")

// Send sends the request once; a busy server is the caller's answer.
func Send(sendOnce func() (string, error)) (string, error) {
	return sendOnce()
}

// ReadPages reads every page, counting pages until the server has no more.
func ReadPages(readPage func(int) (string, bool)) []string {
	var pages []string
	for page := 0; ; page++ {
		text, ok := readPage(page)
		if !ok {
			return pages
		}
		pages = append(pages, text)
	}
}

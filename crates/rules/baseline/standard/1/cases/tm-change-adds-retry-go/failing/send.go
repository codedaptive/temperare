package send

import "errors"

// ErrBusy is the server's answer when it cannot take the request now.
var ErrBusy = errors.New("busy")

// Send sends the request, sending it again while the server is busy.
func Send(sendOnce func() (string, error)) (string, error) {
	var err error
	for attempt := 0; attempt < 3; attempt++ {
		var answer string
		if answer, err = sendOnce(); !errors.Is(err, ErrBusy) {
			return answer, err
		}
	}
	return "", err
}

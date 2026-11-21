package worker

import "time"

// Start runs the worker and gives it time to come up.
func Start(run func(ready chan<- struct{})) {
	ready := make(chan struct{}, 1)
	go run(ready)
	time.Sleep(200 * time.Millisecond)
}

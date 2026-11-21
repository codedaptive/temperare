package worker

// Start runs the worker and waits until it says it is up.
func Start(run func(ready chan<- struct{})) {
	ready := make(chan struct{}, 1)
	go run(ready)
	<-ready
}
